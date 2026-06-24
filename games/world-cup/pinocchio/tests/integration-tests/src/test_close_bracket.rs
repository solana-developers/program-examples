use solana_keypair::Keypair;
use solana_signer::Signer;

use crate::{
    tests::{
        asserts::TransactionResultExt,
        pda::get_bracket_pda,
        utils::{
            chalk_bracket, chalk_bracket_wrong_third, close_bracket, finalize, funded_keypair, init_config, lock,
            post_full_chalk_oracle, refresh_score, set_unix_timestamp, setup, submit_bracket, vault_balance, LOCK_TS,
        },
    },
    WorldCupError,
};

/// Winner (perfect chalk) + loser (misses third place), oracle posted, both refreshed,
/// winner finalized.
fn finalized_winner_and_loser(litesvm: &mut litesvm::LiteSVM, admin: &Keypair) -> (Keypair, Keypair) {
    init_config(litesvm, admin, LOCK_TS).assert_ok();
    let winner = funded_keypair(litesvm);
    let loser = funded_keypair(litesvm);
    submit_bracket(litesvm, &winner, &chalk_bracket(), 87).0.assert_ok();
    submit_bracket(litesvm, &loser, &chalk_bracket_wrong_third(), 87).0.assert_ok();
    set_unix_timestamp(litesvm, LOCK_TS);
    lock(litesvm, admin).assert_ok();
    post_full_chalk_oracle(litesvm, admin, 87);
    let cranker = funded_keypair(litesvm);
    refresh_score(litesvm, &cranker, &winner.pubkey()).assert_ok();
    refresh_score(litesvm, &cranker, &loser.pubkey()).assert_ok();
    finalize(litesvm, admin, &winner.pubkey()).assert_ok();
    (winner, loser)
}

#[test]
fn close_bracket_rolls_rent_into_pot() {
    let (mut litesvm, admin) = setup();
    let (_winner, loser) = finalized_winner_and_loser(&mut litesvm, &admin);

    let (bracket_pda, _) = get_bracket_pda(&loser.pubkey());
    assert!(litesvm.get_account(&bracket_pda).is_some());
    let vault_before = vault_balance(&litesvm);

    // Permissionless: any cranker can close any bracket once finalized.
    let cranker = funded_keypair(&mut litesvm);
    close_bracket(&mut litesvm, &cranker, &loser.pubkey()).assert_ok();

    assert!(vault_balance(&litesvm) > vault_before, "rent rolled into the pot");
    let after = litesvm.get_account(&bracket_pda);
    assert!(after.is_none() || after.unwrap().lamports == 0, "bracket account closed");
}

#[test]
fn cannot_close_before_finalize() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 87).0.assert_ok();
    set_unix_timestamp(&mut litesvm, LOCK_TS);
    lock(&mut litesvm, &admin).assert_ok();

    let cranker = funded_keypair(&mut litesvm);
    close_bracket(&mut litesvm, &cranker, &entrant.pubkey()).assert_err(WorldCupError::InvalidState);
}
