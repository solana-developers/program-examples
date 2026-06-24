use solana_signer::Signer;

use crate::{
    tests::{
        asserts::TransactionResultExt,
        utils::{
            chalk_bracket, funded_keypair, init_config, lock, post_full_chalk_oracle, post_result, read_bracket,
            read_config, refresh_score, set_unix_timestamp, setup, submit_bracket, LOCK_TS,
        },
    },
    WorldCupError, ALL_DECIDED,
};

#[test]
fn refresh_folds_into_tally_once_oracle_complete() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 87).0.assert_ok();

    set_unix_timestamp(&mut litesvm, LOCK_TS);
    lock(&mut litesvm, &admin).assert_ok();
    post_full_chalk_oracle(&mut litesvm, &admin, 87);

    let cranker = funded_keypair(&mut litesvm);
    refresh_score(&mut litesvm, &cranker, &entrant.pubkey()).assert_ok();

    let (score, tally_mask) = read_bracket(&litesvm, &entrant.pubkey());
    assert_eq!(score, 88, "perfect chalk bracket scores the maximum");
    assert_eq!(tally_mask, ALL_DECIDED);

    let view = read_config(&litesvm);
    assert_eq!(view.best_score, 88);
    assert_eq!(view.best_closeness, 0, "guess matched the actual goal total");
    assert_eq!(view.best_index, 0, "the sole entrant is the first submission");
    assert_eq!(view.refreshed_count, 1);
}

#[test]
fn refreshing_an_already_folded_bracket_fails() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 87).0.assert_ok();

    set_unix_timestamp(&mut litesvm, LOCK_TS);
    lock(&mut litesvm, &admin).assert_ok();
    post_full_chalk_oracle(&mut litesvm, &admin, 87);

    let cranker = funded_keypair(&mut litesvm);
    refresh_score(&mut litesvm, &cranker, &entrant.pubkey()).assert_ok();
    // Re-folding the same bracket is rejected (no double-count).
    refresh_score(&mut litesvm, &cranker, &entrant.pubkey()).assert_err(WorldCupError::AlreadyFolded);

    let view = read_config(&litesvm);
    assert_eq!(view.best_index, 0);
    assert_eq!(view.refreshed_count, 1);
}

#[test]
fn refresh_before_oracle_complete_fails() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 87).0.assert_ok();

    set_unix_timestamp(&mut litesvm, LOCK_TS);
    lock(&mut litesvm, &admin).assert_ok();

    // Only the Round of 32 is decided; the oracle is not yet complete.
    let chalk = chalk_bracket();
    for g in 0..16u8 {
        post_result(&mut litesvm, &admin, g, chalk[g as usize]).assert_ok();
    }

    let cranker = funded_keypair(&mut litesvm);
    refresh_score(&mut litesvm, &cranker, &entrant.pubkey()).assert_err(WorldCupError::OracleNotComplete);
}
