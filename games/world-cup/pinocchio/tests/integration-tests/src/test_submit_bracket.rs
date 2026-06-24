use solana_signer::Signer;

use crate::{
    state::common::AccountDiscriminator,
    tests::{
        asserts::TransactionResultExt,
        constants::PROGRAM_ID,
        pda::get_bracket_pda,
        utils::{
            chalk_bracket, funded_keypair, init_config, read_config, set_unix_timestamp, setup, submit_bracket,
            vault_balance, LOCK_TS,
        },
    },
    WorldCupError, ENTRY_FEE,
};

#[test]
fn submit_creates_bracket_and_escrows_stake() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    let vault_before = vault_balance(&litesvm);

    let (result, bracket_pda) = submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 120);
    result.assert_ok();

    let bracket = litesvm.get_account(&bracket_pda).unwrap();
    assert_eq!(bracket.owner, PROGRAM_ID);
    assert_eq!(bracket.data[0], AccountDiscriminator::Bracket as u8);

    assert_eq!(read_config(&litesvm).entrant_count, 1);

    // The vault receives the entry fee minus the bracket account rent.
    let escrowed = vault_balance(&litesvm) - vault_before;
    assert!(escrowed > 0 && escrowed < ENTRY_FEE, "escrowed {escrowed} should be fee minus rent");
    assert!(escrowed > ENTRY_FEE - 5_000_000, "rent should be a small slice of the fee");
}

#[test]
fn submit_rejects_inconsistent_bracket() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    let mut picks = chalk_bracket();
    picks[16] = 9; // game 16 is fed by games 0,1; team 9 never reached it.
    submit_bracket(&mut litesvm, &entrant, &picks, 100).0.assert_err(WorldCupError::InvalidPick);
}

#[test]
fn submit_twice_for_same_wallet_fails() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 100).0.assert_ok();
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 100).0.assert_err(WorldCupError::BracketAlreadyExists);
}

#[test]
fn prefunded_bracket_pda_does_not_block_submit() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    let (bracket_pda, _) = get_bracket_pda(&entrant.pubkey());
    litesvm.airdrop(&bracket_pda, 1_000_000).unwrap();

    let (result, _) = submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 100);
    result.assert_ok();

    let bracket = litesvm.get_account(&bracket_pda).unwrap();
    assert_eq!(bracket.owner, PROGRAM_ID);
    assert_eq!(bracket.data[0], AccountDiscriminator::Bracket as u8);
}

#[test]
fn submit_after_kickoff_is_rejected() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    set_unix_timestamp(&mut litesvm, LOCK_TS);
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 100).0.assert_err(WorldCupError::RegistrationClosed);
}
