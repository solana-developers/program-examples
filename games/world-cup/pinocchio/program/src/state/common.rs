//! Shared account discriminator, tournament state, PDA seeds, and derivation helpers.

use codama::CodamaType;
use pinocchio::{error::ProgramError, Address};

use crate::WorldCupError;

/// Entry fee per bracket, in lamports (0.1 SOL). Covers the bracket account rent;
/// the remainder is escrowed into the pot vault.
pub const ENTRY_FEE: u64 = 100_000_000;

/// PDA seed for the singleton config account.
pub const CONFIG_SEED: &[u8] = b"config";
/// PDA seed for the singleton oracle account.
pub const ORACLE_SEED: &[u8] = b"oracle";
/// PDA seed for the singleton pot vault.
pub const VAULT_SEED: &[u8] = b"vault";
/// PDA seed prefix for per-wallet bracket accounts.
pub const BRACKET_SEED: &[u8] = b"bracket";

/// One-byte discriminator identifying the type of a program-owned account.
///
/// Stored at byte offset 0 of every data-carrying account created by this program.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug, CodamaType)]
pub enum AccountDiscriminator {
    /// [`Config`](super::config::Config) account.
    Config = 0,
    /// [`Oracle`](super::oracle::Oracle) account.
    Oracle = 1,
    /// [`Bracket`](super::bracket::Bracket) account.
    Bracket = 2,
}

impl TryFrom<u8> for AccountDiscriminator {
    type Error = ProgramError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Config),
            1 => Ok(Self::Oracle),
            2 => Ok(Self::Bracket),
            _ => Err(WorldCupError::InvalidAccountDiscriminator.into()),
        }
    }
}

impl From<AccountDiscriminator> for u8 {
    fn from(val: AccountDiscriminator) -> Self {
        val as u8
    }
}

/// Lifecycle state of the tournament, stored in [`Config`](super::config::Config).
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug, CodamaType)]
pub enum TournamentState {
    /// Accepting new brackets.
    Registration = 0,
    /// Kickoff passed: no new brackets; oracle posting + scoring.
    Locked = 1,
    /// Winner set (or tie routed to admin); claim phase.
    Finalized = 2,
}

impl TryFrom<u8> for TournamentState {
    type Error = ProgramError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Registration),
            1 => Ok(Self::Locked),
            2 => Ok(Self::Finalized),
            _ => Err(WorldCupError::InvalidState.into()),
        }
    }
}

/// Finds the singleton config PDA and bump.
pub fn find_config_pda() -> (Address, u8) {
    Address::find_program_address(&[CONFIG_SEED], &crate::ID)
}

/// Finds the singleton oracle PDA and bump.
pub fn find_oracle_pda() -> (Address, u8) {
    Address::find_program_address(&[ORACLE_SEED], &crate::ID)
}

/// Finds the singleton pot vault PDA and bump.
pub fn find_vault_pda() -> (Address, u8) {
    Address::find_program_address(&[VAULT_SEED], &crate::ID)
}

/// Finds the bracket PDA and bump for a wallet.
pub fn find_bracket_pda(owner: &Address) -> (Address, u8) {
    Address::find_program_address(&[BRACKET_SEED, owner.as_ref()], &crate::ID)
}

/// Verifies a bracket PDA by re-deriving with the given owner and bump.
pub fn verify_bracket_pda(owner: &Address, bump: u8) -> Result<Address, ProgramError> {
    Address::create_program_address(&[BRACKET_SEED, owner.as_ref(), &[bump]], &crate::ID)
        .map_err(|_| WorldCupError::InvalidBracketPda.into())
}
