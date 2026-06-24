use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use crate::{
    event_engine::{self, EventSerialize},
    events::ScoreRefreshedEvent,
    instructions::helpers::check_writable,
    state::{bracket::Bracket, common::TournamentState, config::Config, oracle::Oracle},
    tournament::{closeness, score_bracket, ALL_DECIDED},
    WorldCupError,
};

/// Instruction discriminator byte for `RefreshScore`.
pub const DISCRIMINATOR: &u8 = &5;

/// Validated accounts for [`RefreshScore`](crate::WorldCupInstruction::RefreshScore).
pub struct RefreshScoreAccounts<'a> {
    pub config: &'a mut AccountView,
    pub oracle: &'a AccountView,
    pub bracket: &'a mut AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for RefreshScoreAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [config, oracle, bracket, event_authority, self_program] = accounts else {
            return Err(WorldCupError::NotEnoughAccountKeys.into());
        };

        check_writable(config)?;
        Config::check(config)?;
        Oracle::check(oracle)?;
        check_writable(bracket)?;
        Bracket::check(bracket)?;

        Ok(Self { config, oracle, bracket, event_authority, self_program })
    }
}

/// Scores a bracket against the oracle and, once the oracle is complete, folds it
/// exactly once into the provable global tally. Permissionless and idempotent.
pub fn process(accounts: &mut [AccountView]) -> ProgramResult {
    let accounts = RefreshScoreAccounts::try_from(accounts)?;

    {
        let config_data = accounts.config.try_borrow()?;
        let config = Config::load(&config_data)?;
        if TournamentState::try_from(config.state)? != TournamentState::Locked {
            return Err(WorldCupError::InvalidState.into());
        }
    }

    let (results, total_goals) = {
        let oracle_data = accounts.oracle.try_borrow()?;
        let oracle = Oracle::load(&oracle_data)?;
        if !oracle.is_complete() {
            return Err(WorldCupError::OracleNotComplete.into());
        }
        (oracle.results, oracle.total_goals_r32)
    };

    let owner;
    let score;
    {
        let mut bracket_data = accounts.bracket.try_borrow_mut()?;
        let bracket = Bracket::load_mut(&mut bracket_data)?;

        if bracket.tally_mask == ALL_DECIDED {
            return Err(WorldCupError::AlreadyFolded.into());
        }

        score = score_bracket(&bracket.picks, &results);
        bracket.score = score;
        owner = bracket.owner;

        let guess = bracket.tiebreaker_guess;
        let close = closeness(guess, total_goals);
        let index = bracket.entry_index;

        let mut config_data = accounts.config.try_borrow_mut()?;
        let config = Config::load_mut(&mut config_data)?;

        let config_tally_mask = config.tally_mask;
        if config_tally_mask != ALL_DECIDED {
            config.best_score = 0;
            config.best_closeness = u16::MAX;
            config.best_index = u32::MAX;
            config.refreshed_count = 0;
            config.tally_mask = ALL_DECIDED;
        }

        let best_score = config.best_score;
        let best_closeness = config.best_closeness;
        let best_index = config.best_index;
        let better = score > best_score
            || (score == best_score && close < best_closeness)
            || (score == best_score && close == best_closeness && index < best_index);
        if better {
            config.best_score = score;
            config.best_closeness = close;
            config.best_index = index;
        }
        bracket.tally_mask = ALL_DECIDED;
        let refreshed = config.refreshed_count;
        config.refreshed_count = refreshed.checked_add(1).ok_or(WorldCupError::ArithmeticOverflow)?;
    }

    let event = ScoreRefreshedEvent::new(owner, score);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
