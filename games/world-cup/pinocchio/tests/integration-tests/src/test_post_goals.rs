use crate::{
    tests::{
        asserts::TransactionResultExt,
        utils::{funded_keypair, init_and_lock, init_config, post_all_chalk_results, post_goals, setup, LOCK_TS},
    },
    WorldCupError,
};

#[test]
fn admin_posts_goal_total() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);
    post_all_chalk_results(&mut litesvm, &admin);
    post_goals(&mut litesvm, &admin, 87).assert_ok();
}

#[test]
fn goal_total_is_immutable() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);
    post_all_chalk_results(&mut litesvm, &admin);
    post_goals(&mut litesvm, &admin, 87).assert_ok();
    post_goals(&mut litesvm, &admin, 90).assert_err(WorldCupError::GoalsAlreadyPosted);
}

#[test]
fn cannot_post_goals_before_all_results() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);
    post_goals(&mut litesvm, &admin, 87).assert_err(WorldCupError::OracleNotComplete);
}

#[test]
fn non_admin_cannot_post_goals() {
    let (mut litesvm, admin) = setup();
    init_and_lock(&mut litesvm, &admin);
    let stranger = funded_keypair(&mut litesvm);
    post_goals(&mut litesvm, &stranger, 87).assert_err(WorldCupError::Unauthorized);
}

#[test]
fn cannot_post_goals_before_lock() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();
    post_goals(&mut litesvm, &admin, 87).assert_err(WorldCupError::InvalidState);
}
