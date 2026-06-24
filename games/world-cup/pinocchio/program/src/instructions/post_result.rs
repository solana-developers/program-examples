use core::mem::{size_of, transmute};

use codama::CodamaType;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use crate::{
    event_engine::{self, EventSerialize},
    events::ResultPostedEvent,
    instructions::helpers::{check_signer, check_writable},
    state::{common::TournamentState, config::Config, oracle::Oracle},
    tournament::{validate_result, GAME_COUNT, UNDECIDED},
    WorldCupError,
};

/// Instruction discriminator byte for `PostResult`.
pub const DISCRIMINATOR: &u8 = &3;

/// Instruction data for [`PostResult`](crate::WorldCupInstruction::PostResult).
#[repr(C, packed)]
#[derive(CodamaType, Debug, Clone)]
pub struct PostResultData {
    /// Game index `0..32`.
    pub game: u8,
    /// Winning team (positional id `0..32`).
    pub winner: u8,
}

impl PostResultData {
    pub const LEN: usize = size_of::<Self>();

    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(WorldCupError::InvalidInstruction.into());
        }
        Ok(unsafe { &*transmute::<*const u8, *const Self>(data.as_ptr()) })
    }
}

/// Validated accounts for [`PostResult`](crate::WorldCupInstruction::PostResult).
pub struct PostResultAccounts<'a> {
    pub admin: &'a AccountView,
    pub config: &'a AccountView,
    pub oracle: &'a mut AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for PostResultAccounts<'a> {
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

/// Records a single game result on the oracle and emits a [`ResultPostedEvent`].
pub fn process(accounts: &mut [AccountView], data: &PostResultData) -> ProgramResult {
    let accounts = PostResultAccounts::try_from(accounts)?;

    {
        let config_data = accounts.config.try_borrow()?;
        let config = Config::load(&config_data)?;
        config.check_admin(accounts.admin.address())?;
        if TournamentState::try_from(config.state)? != TournamentState::Locked {
            return Err(WorldCupError::InvalidState.into());
        }
    }

    let game = data.game;
    let winner = data.winner;

    let decided_mask;
    {
        let mut oracle_data = accounts.oracle.try_borrow_mut()?;
        let oracle = Oracle::load_mut(&mut oracle_data)?;

        if (game as usize) >= GAME_COUNT {
            return Err(WorldCupError::InvalidGame.into());
        }
        if oracle.results[game as usize] != UNDECIDED {
            return Err(WorldCupError::ResultAlreadyPosted.into());
        }

        validate_result(&oracle.results, game, winner)?;

        oracle.results[game as usize] = winner;
        let current_mask = oracle.decided_mask;
        decided_mask = current_mask | (1u32 << game);
        oracle.decided_mask = decided_mask;
    }

    let event = ResultPostedEvent::new(game, winner, decided_mask);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
