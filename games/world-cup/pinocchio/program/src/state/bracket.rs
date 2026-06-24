//! Per-wallet bracket account: the entrant's 32 picks, tiebreaker, and cached score.

use codama::CodamaAccount;
use core::mem::{size_of, transmute};
use pinocchio::{error::ProgramError, AccountView, Address};

use crate::{state::common::AccountDiscriminator, tournament::GAME_COUNT, WorldCupError};

/// One entrant's bracket.
///
/// `score` is the cached weighted score from the last `refresh_score`. `tally_mask`
/// is the oracle `decided_mask` this bracket was last folded into the global tally
/// at, making the fold idempotent (a bracket already folded at the current mask is
/// skipped) and self-invalidating when the oracle advances.
///
/// **PDA seeds:** `["bracket", owner]`
#[repr(C, packed)]
#[derive(CodamaAccount)]
#[codama(seed(type = string(utf8), value = "bracket"))]
#[codama(seed(name = "owner", type = public_key))]
pub struct Bracket {
    /// Account type discriminator ([`AccountDiscriminator::Bracket`]).
    pub discriminator: u8,
    /// PDA bump seed.
    pub bump: u8,
    /// Wallet that submitted (and owns) this bracket.
    pub owner: Address,
    /// Winner pick per game (positional team id `0..32`).
    pub picks: [u8; 32],
    /// Predicted total Round-of-32 goals (tiebreaker guess).
    pub tiebreaker_guess: u16,
    /// Cached weighted score from the last refresh.
    pub score: u16,
    /// Oracle `decided_mask` this bracket was last folded into the tally at.
    pub tally_mask: u32,
    /// Zero-based submission order, assigned at submit. Final ranking tiebreaker.
    pub entry_index: u32,
}

impl Bracket {
    /// Total serialized size in bytes.
    pub const LEN: usize = size_of::<Self>();

    /// PDA seed prefix.
    pub const SEED: &'static [u8] = b"bracket";

    /// Initializes a freshly created account.
    #[inline(always)]
    pub fn init(
        bytes: &mut [u8],
        bump: u8,
        owner: &Address,
        picks: &[u8; GAME_COUNT],
        tiebreaker_guess: u16,
        entry_index: u32,
    ) -> Result<(), ProgramError> {
        if bytes.len() != Self::LEN {
            return Err(WorldCupError::InvalidAccountData.into());
        }
        let account = unsafe { &mut *transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) };
        account.discriminator = AccountDiscriminator::Bracket as u8;
        account.bump = bump;
        account.owner = *owner;
        account.picks = *picks;
        account.tiebreaker_guess = tiebreaker_guess;
        account.score = 0;
        account.tally_mask = 0;
        account.entry_index = entry_index;
        Ok(())
    }

    /// Deserializes a mutable reference from raw account data.
    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        Self::validate(bytes)?;
        Ok(unsafe { &mut *transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    /// Deserializes an immutable reference from raw account data.
    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        Self::validate(bytes)?;
        Ok(unsafe { &*transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    }

    /// Verifies an account is a genuine bracket: owned by this program, correctly
    /// sized, and carrying the bracket discriminator. Call in account validation.
    pub fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.owned_by(&crate::ID) {
            return Err(WorldCupError::NotProgramOwned.into());
        }
        Self::validate(&account.try_borrow()?)
    }

    fn validate(bytes: &[u8]) -> Result<(), ProgramError> {
        if bytes.len() != Self::LEN {
            return Err(WorldCupError::InvalidAccountData.into());
        }
        if bytes[0] != AccountDiscriminator::Bracket as u8 {
            return Err(WorldCupError::InvalidAccountDiscriminator.into());
        }
        Ok(())
    }
}
