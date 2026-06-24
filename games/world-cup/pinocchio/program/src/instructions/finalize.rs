use pinocchio::{error::ProgramError, AccountView, ProgramResult};

use crate::{
    event_engine::{self, EventSerialize},
    events::MarketFinalizedEvent,
    instructions::helpers::{check_signer, check_writable},
    state::{
        bracket::Bracket,
        common::{verify_bracket_pda, TournamentState},
        config::Config,
        oracle::Oracle,
    },
    tournament::{closeness, ALL_DECIDED},
    WorldCupError,
};

/// Instruction discriminator byte for `Finalize`.
pub const DISCRIMINATOR: &u8 = &6;

/// Validated accounts for [`Finalize`](crate::WorldCupInstruction::Finalize).
pub struct FinalizeAccounts<'a> {
    pub admin: &'a AccountView,
    pub config: &'a mut AccountView,
    pub oracle: &'a AccountView,
    pub bracket: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for FinalizeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [admin, config, oracle, bracket, event_authority, self_program] = accounts else {
            return Err(WorldCupError::NotEnoughAccountKeys.into());
        };

        check_signer(admin)?;
        check_writable(config)?;
        Config::check(config)?;
        Oracle::check(oracle)?;
        Bracket::check(bracket)?;

        Ok(Self { admin, config, oracle, bracket, event_authority, self_program })
    }
}

/// Records the unique provable winner and emits a [`MarketFinalizedEvent`].
///
/// Trust-minimized: the program proves the passed bracket matches the on-chain best
/// `(score, closeness, index)` key and that every bracket has been folded into the
/// tally at the final state. The submission index makes the key total, so exactly one
/// bracket can match it.
pub fn process(accounts: &mut [AccountView]) -> ProgramResult {
    let accounts = FinalizeAccounts::try_from(accounts)?;

    let (best_score, best_closeness, best_index, tally_mask, refreshed_count, entrant_count) = {
        let config_data = accounts.config.try_borrow()?;
        let config = Config::load(&config_data)?;
        config.check_admin(accounts.admin.address())?;
        if TournamentState::try_from(config.state)? != TournamentState::Locked {
            return Err(WorldCupError::InvalidState.into());
        }
        (
            config.best_score,
            config.best_closeness,
            config.best_index,
            config.tally_mask,
            config.refreshed_count,
            config.entrant_count,
        )
    };

    let total_goals = {
        let oracle_data = accounts.oracle.try_borrow()?;
        let oracle = Oracle::load(&oracle_data)?;
        if !oracle.is_complete() {
            return Err(WorldCupError::OracleNotComplete.into());
        }
        oracle.total_goals_r32
    };

    if tally_mask != ALL_DECIDED || refreshed_count != entrant_count {
        return Err(WorldCupError::NotFullyRefreshed.into());
    }

    let (winner, bracket_bump, bracket_score, bracket_guess, bracket_tally, bracket_index) = {
        let bracket_data = accounts.bracket.try_borrow()?;
        let bracket = Bracket::load(&bracket_data)?;
        (bracket.owner, bracket.bump, bracket.score, bracket.tiebreaker_guess, bracket.tally_mask, bracket.entry_index)
    };

    let expected_pda = verify_bracket_pda(&winner, bracket_bump)?;
    if expected_pda != *accounts.bracket.address() {
        return Err(WorldCupError::InvalidBracketPda.into());
    }

    let bracket_closeness = closeness(bracket_guess, total_goals);
    if bracket_tally != ALL_DECIDED
        || bracket_score != best_score
        || bracket_closeness != best_closeness
        || bracket_index != best_index
    {
        return Err(WorldCupError::BracketNotBest.into());
    }

    {
        let mut config_data = accounts.config.try_borrow_mut()?;
        let config = Config::load_mut(&mut config_data)?;
        config.winner = winner;
        config.state = TournamentState::Finalized as u8;
    }

    let event = MarketFinalizedEvent::new(winner, best_score, best_closeness);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
