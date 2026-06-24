use pinocchio::{
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    AccountView, ProgramResult,
};

use crate::{
    event_engine::{self, EventSerialize},
    events::TournamentLockedEvent,
    instructions::helpers::{check_signer, check_writable},
    state::{common::TournamentState, config::Config},
    WorldCupError,
};

/// Instruction discriminator byte for `Lock`.
pub const DISCRIMINATOR: &u8 = &2;

/// Validated accounts for [`Lock`](crate::WorldCupInstruction::Lock).
pub struct LockAccounts<'a> {
    pub admin: &'a AccountView,
    pub config: &'a mut AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for LockAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [admin, config, event_authority, self_program] = accounts else {
            return Err(WorldCupError::NotEnoughAccountKeys.into());
        };

        check_signer(admin)?;
        check_writable(config)?;
        Config::check(config)?;

        Ok(Self { admin, config, event_authority, self_program })
    }
}

/// Transitions the tournament from registration to locked once `lock_ts` is reached.
pub fn process(accounts: &mut [AccountView]) -> ProgramResult {
    let accounts = LockAccounts::try_from(accounts)?;
    let now = Clock::get()?.unix_timestamp;

    let lock_ts;
    {
        let mut config_data = accounts.config.try_borrow_mut()?;
        let config = Config::load_mut(&mut config_data)?;
        config.check_admin(accounts.admin.address())?;
        if TournamentState::try_from(config.state)? != TournamentState::Registration {
            return Err(WorldCupError::InvalidState.into());
        }
        lock_ts = config.lock_ts;
        if now < lock_ts {
            return Err(WorldCupError::NotYetLocked.into());
        }
        config.state = TournamentState::Locked as u8;
    }

    let event = TournamentLockedEvent::new(lock_ts);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
