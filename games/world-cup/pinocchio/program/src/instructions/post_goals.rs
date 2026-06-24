use core::mem::{size_of, transmute};

use codama::CodamaType;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use crate::{
    event_engine::{self, EventSerialize},
    events::GoalsPostedEvent,
    instructions::helpers::{check_signer, check_writable},
    state::{common::TournamentState, config::Config, oracle::Oracle},
    tournament::ALL_DECIDED,
    WorldCupError,
};

/// Instruction discriminator byte for `PostGoals`.
pub const DISCRIMINATOR: &u8 = &4;

/// Instruction data for [`PostGoals`](crate::WorldCupInstruction::PostGoals).
#[repr(C, packed)]
#[derive(CodamaType, Debug, Clone)]
pub struct PostGoalsData {
    /// Actual total goals scored across the Round of 32.
    pub total_goals_r32: u16,
}

impl PostGoalsData {
    pub const LEN: usize = size_of::<Self>();

    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(WorldCupError::InvalidInstruction.into());
        }
        Ok(unsafe { &*transmute::<*const u8, *const Self>(data.as_ptr()) })
    }
}

/// Validated accounts for [`PostGoals`](crate::WorldCupInstruction::PostGoals).
pub struct PostGoalsAccounts<'a> {
    pub admin: &'a AccountView,
    pub config: &'a AccountView,
    pub oracle: &'a mut AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for PostGoalsAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [admin, config, oracle, event_authority, self_program] = accounts else {
            return Err(WorldCupError::NotEnoughAccountKeys.into());
        };

        check_signer(admin)?;
        Config::check(config)?;
        check_writable(oracle)?;
        Oracle::check(oracle)?;

        Ok(Self { admin, config, oracle, event_authority, self_program })
    }
}

/// Records the Round-of-32 goal total and emits a [`GoalsPostedEvent`].
pub fn process(accounts: &mut [AccountView], data: &PostGoalsData) -> ProgramResult {
    let accounts = PostGoalsAccounts::try_from(accounts)?;

    {
        let config_data = accounts.config.try_borrow()?;
        let config = Config::load(&config_data)?;
        config.check_admin(accounts.admin.address())?;
        if TournamentState::try_from(config.state)? != TournamentState::Locked {
            return Err(WorldCupError::InvalidState.into());
        }
    }

    let total = data.total_goals_r32;
    {
        let mut oracle_data = accounts.oracle.try_borrow_mut()?;
        let oracle = Oracle::load_mut(&mut oracle_data)?;
        if oracle.goals_posted != 0 {
            return Err(WorldCupError::GoalsAlreadyPosted.into());
        }
        let decided_mask = oracle.decided_mask;
        if decided_mask != ALL_DECIDED {
            return Err(WorldCupError::OracleNotComplete.into());
        }
        oracle.total_goals_r32 = total;
        oracle.goals_posted = 1;
    }

    let event = GoalsPostedEvent::new(total);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
