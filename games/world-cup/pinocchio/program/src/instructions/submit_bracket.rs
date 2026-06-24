use core::mem::{size_of, transmute};

use codama::CodamaType;
use pinocchio::{
    cpi::Seed,
    error::ProgramError,
    sysvars::{clock::Clock, rent::Rent, Sysvar},
    AccountView, ProgramResult,
};
use pinocchio_system::instructions::Transfer;

use crate::{
    event_engine::{self, EventSerialize},
    events::BracketSubmittedEvent,
    instructions::helpers::{check_signer, check_system_program, check_writable},
    state::{
        bracket::Bracket,
        common::{find_bracket_pda, find_vault_pda, TournamentState, BRACKET_SEED},
        config::Config,
    },
    tournament::validate_bracket,
    WorldCupError,
};

/// Instruction discriminator byte for `SubmitBracket`.
pub const DISCRIMINATOR: &u8 = &1;

/// Instruction data for [`SubmitBracket`](crate::WorldCupInstruction::SubmitBracket).
#[repr(C, packed)]
#[derive(CodamaType, Debug, Clone)]
pub struct SubmitBracketData {
    /// Winner pick per game (positional team id `0..32`).
    pub picks: [u8; 32],
    /// Predicted total Round-of-32 goals (tiebreaker guess).
    pub tiebreaker_guess: u16,
}

impl SubmitBracketData {
    pub const LEN: usize = size_of::<Self>();

    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(WorldCupError::InvalidInstruction.into());
        }
        Ok(unsafe { &*transmute::<*const u8, *const Self>(data.as_ptr()) })
    }
}

/// Validated accounts for [`SubmitBracket`](crate::WorldCupInstruction::SubmitBracket).
pub struct SubmitBracketAccounts<'a> {
    pub entrant: &'a AccountView,
    pub config: &'a mut AccountView,
    pub bracket: &'a mut AccountView,
    pub vault: &'a AccountView,
    pub system_program: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for SubmitBracketAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [entrant, config, bracket, vault, system_program, event_authority, self_program] = accounts else {
            return Err(WorldCupError::NotEnoughAccountKeys.into());
        };

        check_signer(entrant)?;
        check_writable(entrant)?;
        check_writable(config)?;
        Config::check(config)?;
        check_writable(bracket)?;
        check_writable(vault)?;
        check_system_program(system_program)?;

        Ok(Self { entrant, config, bracket, vault, system_program, event_authority, self_program })
    }
}

/// Creates a [`Bracket`] for the entrant, escrows the stake, and emits a [`BracketSubmittedEvent`].
pub fn process(accounts: &mut [AccountView], data: &SubmitBracketData) -> ProgramResult {
    let accounts = SubmitBracketAccounts::try_from(accounts)?;
    let now = Clock::get()?.unix_timestamp;

    let entry_fee;
    let entry_index;
    {
        let config_data = accounts.config.try_borrow()?;
        let config = Config::load(&config_data)?;
        if TournamentState::try_from(config.state)? != TournamentState::Registration {
            return Err(WorldCupError::InvalidState.into());
        }
        let lock_ts = config.lock_ts;
        if now >= lock_ts {
            return Err(WorldCupError::RegistrationClosed.into());
        }
        entry_fee = config.entry_fee;
        entry_index = config.entrant_count;
    }

    if accounts.bracket.data_len() > 0 {
        return Err(WorldCupError::BracketAlreadyExists.into());
    }

    let (bracket_pda, bump) = find_bracket_pda(accounts.entrant.address());
    if bracket_pda != *accounts.bracket.address() {
        return Err(WorldCupError::InvalidBracketPda.into());
    }

    if find_vault_pda().0 != *accounts.vault.address() {
        return Err(WorldCupError::InvalidVaultPda.into());
    }

    validate_bracket(&data.picks)?;

    let bracket_rent = Rent::get()?.try_minimum_balance(Bracket::LEN)?;
    let pot_contribution = entry_fee.checked_sub(bracket_rent).ok_or(WorldCupError::ArithmeticOverflow)?;

    let bump_bytes = [bump];
    let seeds =
        [Seed::from(BRACKET_SEED), Seed::from(accounts.entrant.address().as_ref()), Seed::from(&bump_bytes[..])];
    crate::instructions::helpers::create_pda_account(accounts.entrant, accounts.bracket, &seeds, Bracket::LEN)?;

    Transfer { from: accounts.entrant, to: accounts.vault, lamports: pot_contribution }.invoke()?;

    {
        let mut bracket_data = accounts.bracket.try_borrow_mut()?;
        Bracket::init(
            &mut bracket_data,
            bump,
            accounts.entrant.address(),
            &data.picks,
            data.tiebreaker_guess,
            entry_index,
        )?;
    }

    let entrant_count;
    {
        let mut config_data = accounts.config.try_borrow_mut()?;
        let config = Config::load_mut(&mut config_data)?;
        let current = config.entrant_count;
        entrant_count = current.checked_add(1).ok_or(WorldCupError::ArithmeticOverflow)?;
        config.entrant_count = entrant_count;
    }

    let event =
        BracketSubmittedEvent::new(*accounts.entrant.address(), entrant_count, data.tiebreaker_guess, data.picks);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
