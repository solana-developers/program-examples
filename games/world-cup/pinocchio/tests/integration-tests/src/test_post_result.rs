use crate::{
    tests::{
        asserts::TransactionResultExt,
        utils::{funded_keypair, init_and_lock, init_config, oracle_decided_mask, post_result, setup, LOCK_TS},
    },
    WorldCupError,
};

#[test]
fn admin_posts_result_and_sets_decided_bit() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);

    // Round-of-32 game 0 is contested by teams 0 and 1.
    post_result(&mut litesvm, &admin, 0, 0).assert_ok();
    assert_eq!(oracle_decided_mask(&litesvm), 1, "bit 0 set");

    post_result(&mut litesvm, &admin, 1, 2).assert_ok();
    assert_eq!(oracle_decided_mask(&litesvm), 0b11, "bits 0 and 1 set");
}

#[test]
fn non_admin_cannot_post() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);

    let stranger = funded_keypair(&mut litesvm);
    post_result(&mut litesvm, &stranger, 0, 0).assert_err(WorldCupError::Unauthorized);
}

#[test]
fn cannot_post_dependent_before_feeders() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);

    // Game 16 is fed by games 0 and 1, neither decided yet.
    post_result(&mut litesvm, &admin, 16, 0).assert_err(WorldCupError::FeederNotDecided);
}

#[test]
fn rejects_winner_not_in_the_contest() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);

    // Game 0 is teams 0,1; team 5 cannot win it.
    post_result(&mut litesvm, &admin, 0, 5).assert_err(WorldCupError::InvalidResult);
}

#[test]
fn results_are_immutable() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);

    post_result(&mut litesvm, &admin, 0, 0).assert_ok();
    post_result(&mut litesvm, &admin, 0, 1).assert_err(WorldCupError::ResultAlreadyPosted);
}

#[test]
fn cannot_post_before_lock() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    post_result(&mut litesvm, &admin, 0, 0).assert_err(WorldCupError::InvalidState);
}
