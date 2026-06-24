use crate::{
    tests::{
        asserts::TransactionResultExt,
        utils::{
            chalk_bracket, funded_keypair, init_config, lock, read_config, set_unix_timestamp, setup, submit_bracket,
            LOCK_TS,
        },
    },
    TournamentState, WorldCupError,
};

#[test]
fn lock_before_kickoff_fails() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    // Still at BASE_TS, well before LOCK_TS.
    lock(&mut litesvm, &admin).assert_err(WorldCupError::NotYetLocked);
}

#[test]
fn lock_after_kickoff_transitions_state() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    set_unix_timestamp(&mut litesvm, LOCK_TS);
    lock(&mut litesvm, &admin).assert_ok();
    assert_eq!(read_config(&litesvm).state, TournamentState::Locked as u8);
}

#[test]
fn lock_by_non_admin_fails() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    set_unix_timestamp(&mut litesvm, LOCK_TS);
    let stranger = funded_keypair(&mut litesvm);
    lock(&mut litesvm, &stranger).assert_err(WorldCupError::Unauthorized);
}

#[test]
fn submit_after_lock_is_rejected() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    set_unix_timestamp(&mut litesvm, LOCK_TS);
    lock(&mut litesvm, &admin).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 100).0.assert_err(WorldCupError::InvalidState);
}

#[test]
fn lock_twice_fails() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    set_unix_timestamp(&mut litesvm, LOCK_TS);
    lock(&mut litesvm, &admin).assert_ok();
    lock(&mut litesvm, &admin).assert_err(WorldCupError::InvalidState);
}
