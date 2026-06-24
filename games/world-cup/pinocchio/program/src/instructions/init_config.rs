use core::mem::{size_of, transmute};

use codama::CodamaType;
use pinocchio::{
    cpi::Seed,
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    AccountView, ProgramResult,
};

use crate::{
    event_engine::{self, EventSerialize},
    events::ConfigInitializedEvent,
    instructions::helpers::{check_signer, check_system_program, check_writable, create_pda_account},
    state::{
        common::{find_config_pda, find_oracle_pda, find_vault_pda, CONFIG_SEED, ENTRY_FEE, ORACLE_SEED, VAULT_SEED},
        config::Config,
        oracle::Oracle,
    },
    WorldCupError,
};

/// Instruction discriminator byte for `InitConfig`.
pub const DISCRIMINATOR: &u8 = &0;

/// Instruction data for [`InitConfig`](crate::WorldCupInstruction::InitConfig).
#[repr(C, packed)]
#[derive(CodamaType, Debug, Clone)]
pub struct InitConfigData {
    /// Unix timestamp at/after which registration closes and `lock` may be called.
    pub lock_ts: i64,
}

impl InitConfigData {
    pub const LEN: usize = size_of::<Self>();

    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(WorldCupError::InvalidInstruction.into());
        }
        Ok(unsafe { &*transmute::<*const u8, *const Self>(data.as_ptr()) })
    }
}

/// Validated accounts for [`InitConfig`](crate::WorldCupInstruction::InitConfig).
pub struct InitConfigAccounts<'a> {
    pub admin: &'a AccountView,
    pub config: &'a mut AccountView,
    pub oracle: &'a mut AccountView,
    pub vault: &'a AccountView,
    pub system_program: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub self_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for InitConfigAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [admin, config, oracle, vault, system_program, event_authority, self_program] = accounts else {
            return Err(WorldCupError::NotEnoughAccountKeys.into());
        };

        check_signer(admin)?;
        check_writable(admin)?;
        check_writable(config)?;
        check_writable(oracle)?;
        check_writable(vault)?;
        check_system_program(system_program)?;

        Ok(Self { admin, config, oracle, vault, system_program, event_authority, self_program })
    }
}

/// Creates the singleton config, oracle, and pot vault, and emits a [`ConfigInitializedEvent`].
pub fn process(accounts: &mut [AccountView], data: &InitConfigData) -> ProgramResult {
    let accounts = InitConfigAccounts::try_from(accounts)?;

    let now = Clock::get()?.unix_timestamp;
    if data.lock_ts <= now {
        return Err(WorldCupError::InvalidLockTs.into());
    }

    if accounts.config.data_len() > 0 {
        return Err(WorldCupError::ConfigAlreadyExists.into());
    }

    let (config_pda, config_bump) = find_config_pda();
    if config_pda != *accounts.config.address() {
        return Err(WorldCupError::InvalidConfigPda.into());
    }
    let (oracle_pda, oracle_bump) = find_oracle_pda();
    if oracle_pda != *accounts.oracle.address() {
        return Err(WorldCupError::InvalidOraclePda.into());
    }
    let (vault_pda, vault_bump) = find_vault_pda();
    if vault_pda != *accounts.vault.address() {
        return Err(WorldCupError::InvalidVaultPda.into());
    }

    let lock_ts = data.lock_ts;

    let config_bump_bytes = [config_bump];
    let config_seeds = [Seed::from(CONFIG_SEED), Seed::from(&config_bump_bytes[..])];
    create_pda_account(accounts.admin, accounts.config, &config_seeds, Config::LEN)?;

    let oracle_bump_bytes = [oracle_bump];
    let oracle_seeds = [Seed::from(ORACLE_SEED), Seed::from(&oracle_bump_bytes[..])];
    create_pda_account(accounts.admin, accounts.oracle, &oracle_seeds, Oracle::LEN)?;

    let vault_bump_bytes = [vault_bump];
    let vault_seeds = [Seed::from(VAULT_SEED), Seed::from(&vault_bump_bytes[..])];
    create_pda_account(accounts.admin, accounts.vault, &vault_seeds, 0)?;

    {
        let mut config_data = accounts.config.try_borrow_mut()?;
        Config::init(&mut config_data, config_bump, accounts.admin.address(), lock_ts, ENTRY_FEE)?;
    }
    {
        let mut oracle_data = accounts.oracle.try_borrow_mut()?;
        Oracle::init(&mut oracle_data, oracle_bump)?;
    }

    let event = ConfigInitializedEvent::new(*accounts.admin.address(), lock_ts, ENTRY_FEE);
    event_engine::emit_event(&crate::ID, accounts.event_authority, accounts.self_program, &event.to_bytes())?;

    Ok(())
}
