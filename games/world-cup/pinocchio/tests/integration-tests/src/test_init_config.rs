use crate::{
    state::common::AccountDiscriminator,
    tests::{
        asserts::TransactionResultExt,
        constants::PROGRAM_ID,
        pda::{get_config_pda, get_oracle_pda, get_vault_pda},
        utils::{init_config, read_config, setup, BASE_TS, LOCK_TS},
    },
    Oracle, TournamentState, WorldCupError, ENTRY_FEE,
};

#[test]
fn init_config_creates_singletons() {
    let (mut litesvm, admin) = setup();

    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let (config_pda, _) = get_config_pda();
    let config = litesvm.get_account(&config_pda).unwrap();
    assert_eq!(config.owner, PROGRAM_ID);
    assert_eq!(config.data[0], AccountDiscriminator::Config as u8);

    let view = read_config(&litesvm);
    assert_eq!(view.state, TournamentState::Registration as u8);
    assert_eq!(view.entrant_count, 0);
    assert_eq!(view.winner, [0u8; 32]);

    let (oracle_pda, _) = get_oracle_pda();
    let oracle_account = litesvm.get_account(&oracle_pda).unwrap();
    assert_eq!(oracle_account.data[0], AccountDiscriminator::Oracle as u8);
    let oracle = Oracle::load(&oracle_account.data).unwrap();
    let decided = oracle.decided_mask;
    assert_eq!(decided, 0, "no games decided at init");
    assert!(oracle.results.iter().all(|&r| r == 0xFF), "all games undecided");

    let (vault_pda, _) = get_vault_pda();
    let vault = litesvm.get_account(&vault_pda).unwrap();
    assert_eq!(vault.owner, PROGRAM_ID, "vault is program-owned");
    assert!(vault.lamports > 0, "vault is rent-funded");

    // Entry fee is the fixed protocol constant.
    assert_eq!(ENTRY_FEE, 100_000_000);
}

#[test]
fn init_config_twice_fails() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_err(crate::WorldCupError::ConfigAlreadyExists);
}

#[test]
fn init_config_rejects_past_lock_ts() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, BASE_TS).assert_err(WorldCupError::InvalidLockTs);
}
