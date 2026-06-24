//! Singleton oracle account: admin-posted game results and Round-of-32 goal total.

use codama::CodamaAccount;
use core::mem::{size_of, transmute};
use pinocchio::{error::ProgramError, AccountView};

use crate::{
    state::common::AccountDiscriminator,
    tournament::{ALL_DECIDED, GAME_COUNT, UNDECIDED},
    WorldCupError,
};

/// The singleton oracle: the recorded truth of the tournament.
///
/// `results[g]` is the winning team of game `g`, or [`UNDECIDED`] until posted.
/// Results are immutable once set. `total_goals_r32` is posted separately (it is not
/// derivable from winners) and drives the tiebreaker.
///
/// **PDA seeds:** `["oracle"]`
#[repr(C, packed)]
#[derive(CodamaAccount)]
#[codama(seed(type = string(utf8), value = "oracle"))]
pub struct Oracle {
    /// Account type discriminator ([`AccountDiscriminator::Oracle`]).
    pub discriminator: u8,
    /// PDA bump seed.
    pub bump: u8,
    /// `1` once [`total_goals_r32`](Self::total_goals_r32) has been posted.
    pub goals_posted: u8,
    /// Winning team per game; [`UNDECIDED`] until posted.
    pub results: [u8; 32],
    /// Bitmap of decided games; `ALL_DECIDED` when every game is in.
    pub decided_mask: u32,
    /// Actual total goals scored across the Round of 32 (the tiebreaker target).
    pub total_goals_r32: u16,
}

impl Oracle {
    /// Total serialized size in bytes.
    pub const LEN: usize = size_of::<Self>();

    /// PDA seed prefix.
    pub const SEED: &'static [u8] = b"oracle";

    /// Initializes a freshly created account with all games undecided.
    #[inline(always)]
    pub fn init(bytes: &mut [u8], bump: u8) -> Result<(), ProgramError> {
        if bytes.len() != Self::LEN {
            return Err(WorldCupError::InvalidAccountData.into());
        }
        let account = unsafe { &mut *transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) };
        account.discriminator = AccountDiscriminator::Oracle as u8;
        account.bump = bump;
        account.goals_posted = 0;
        account.results = [UNDECIDED; GAME_COUNT];
        account.decided_mask = 0;
        account.total_goals_r32 = 0;
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

    /// Verifies an account is a genuine oracle: owned by this program, correctly
    /// sized, and carrying the oracle discriminator. Call in account validation.
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
        if bytes[0] != AccountDiscriminator::Oracle as u8 {
            return Err(WorldCupError::InvalidAccountDiscriminator.into());
        }
        Ok(())
    }

    /// Whether every game is decided and the goal total has been posted.
    #[inline]
    pub fn is_complete(&self) -> bool {
        self.decided_mask == ALL_DECIDED && self.goals_posted != 0
    }
}
