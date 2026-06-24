use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use crate::{
    event_engine::{self, EventSerialize},
    events::BracketClosedEvent,
    instructions::helpers::{check_writable, close_account},
    state::{
        bracket::Bracket,
        common::{find_vault_pda, verify_bracket_pda, TournamentState},
        config::Config,
    },
    WorldCupError,
};

/// Instruction discriminator byte for `CloseBracket`.
pub const DISCRIMINATOR: &u8 = &8;

/// Validated accounts for [`CloseBracket`](crate::WorldCupInstruction::CloseBracket).
pub struct CloseBracketAccounts<'a> {
    pub config: &'a AccountView,
    pub bracket: &'a AccountView,
    pub vault: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for CloseBracketAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [config, bracket, vault, event_authority, self_program] = accounts else {
            return Err(WorldCupError::NotEnoughAccountKeys.into());
        };

        Config::check(config)?;
        check_writable(bracket)?;
        Bracket::check(bracket)?;
        check_writable(vault)?;

        Ok(Self { config, bracket, vault, event_authority, self_program })
    }
}

/// Permissionlessly closes any bracket once the tournament is finalized, rolling its
/// rent into the pot vault, and emits a [`BracketClosedEvent`].
pub fn process(accounts: &mut [AccountView]) -> ProgramResult {
    let accounts = CloseBracketAccounts::try_from(accounts)?;

    {
        let config_data = accounts.config.try_borrow()?;
        let config = Config::load(&config_data)?;
        if TournamentState::try_from(config.state)? != TournamentState::Finalized {
            return Err(WorldCupError::InvalidState.into());
        }
    }

    let (owner, bump) = {
        let bracket_data = accounts.bracket.try_borrow()?;
        let bracket = Bracket::load(&bracket_data)?;
        (bracket.owner, bracket.bump)
    };

    let expected_pda = verify_bracket_pda(&owner, bump)?;
    if expected_pda != *accounts.bracket.address() {
        return Err(WorldCupError::InvalidBracketPda.into());
    }
    if find_vault_pda().0 != *accounts.vault.address() {
        return Err(WorldCupError::InvalidVaultPda.into());
    }

    close_account(accounts.bracket, accounts.vault)?;

    let event = BracketClosedEvent::new(owner);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
