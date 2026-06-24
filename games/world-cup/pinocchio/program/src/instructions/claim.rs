use pinocchio::{
    error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    AccountView, ProgramResult,
};

use crate::{
    event_engine::{self, EventSerialize},
    events::PotClaimedEvent,
    instructions::helpers::{check_signer, check_writable},
    state::{
        common::{find_vault_pda, TournamentState},
        config::Config,
    },
    WorldCupError,
};

/// Instruction discriminator byte for `Claim`.
pub const DISCRIMINATOR: &u8 = &7;

/// Validated accounts for [`Claim`](crate::WorldCupInstruction::Claim).
pub struct ClaimAccounts<'a> {
    pub winner: &'a AccountView,
    pub config: &'a mut AccountView,
    pub vault: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for ClaimAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [winner, config, vault, event_authority, self_program] = accounts else {
            return Err(WorldCupError::NotEnoughAccountKeys.into());
        };

        check_signer(winner)?;
        check_writable(winner)?;
        check_writable(config)?;
        Config::check(config)?;
        check_writable(vault)?;

        Ok(Self { winner, config, vault, event_authority, self_program })
    }
}

/// Sweeps the available pot to the recorded unique winner and emits a [`PotClaimedEvent`].
///
/// Repeatable: the vault is left alive at its rent floor so bracket rents that flow in
/// via later `close_bracket` calls can be swept by the winner in a follow-up `claim`.
pub fn process(accounts: &mut [AccountView]) -> ProgramResult {
    let accounts = ClaimAccounts::try_from(accounts)?;

    {
        let config_data = accounts.config.try_borrow()?;
        let config = Config::load(&config_data)?;
        if TournamentState::try_from(config.state)? != TournamentState::Finalized {
            return Err(WorldCupError::InvalidState.into());
        }
        let recorded = config.winner;
        if recorded != *accounts.winner.address() {
            return Err(WorldCupError::NotWinner.into());
        }
    }

    if find_vault_pda().0 != *accounts.vault.address() {
        return Err(WorldCupError::InvalidVaultPda.into());
    }

    let floor = Rent::get()?.try_minimum_balance(0)?;
    let amount = accounts.vault.lamports().saturating_sub(floor);
    {
        let mut vault = *accounts.vault;
        let mut winner = *accounts.winner;
        let new_balance = winner.lamports().checked_add(amount).ok_or(WorldCupError::ArithmeticOverflow)?;
        winner.set_lamports(new_balance);
        vault.set_lamports(floor);
    }

    let event = PotClaimedEvent::new(*accounts.winner.address(), amount);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
