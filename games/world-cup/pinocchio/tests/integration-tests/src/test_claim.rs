use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_signer::Signer;

use crate::{
    tests::{
        asserts::TransactionResultExt,
        constants::{EVENT_AUTHORITY, PROGRAM_ID, SYSTEM_PROGRAM_ID},
        pda::get_vault_pda,
        utils::{
            build_and_send, chalk_bracket, claim, finalize, funded_keypair, init_config, lock, post_full_chalk_oracle,
            refresh_score, set_unix_timestamp, setup, submit_bracket, vault_balance, LOCK_TS,
        },
    },
    Config, WorldCupError,
};

/// Runs a single-entrant tournament through finalize; returns the winner keypair.
fn finalized_single_winner(litesvm: &mut litesvm::LiteSVM, admin: &Keypair) -> Keypair {
    init_config(litesvm, admin, LOCK_TS).assert_ok();
    let winner = funded_keypair(litesvm);
    submit_bracket(litesvm, &winner, &chalk_bracket(), 87).0.assert_ok();
    set_unix_timestamp(litesvm, LOCK_TS);
    lock(litesvm, admin).assert_ok();
    post_full_chalk_oracle(litesvm, admin, 87);
    let cranker = funded_keypair(litesvm);
    refresh_score(litesvm, &cranker, &winner.pubkey()).assert_ok();
    finalize(litesvm, admin, &winner.pubkey()).assert_ok();
    winner
}

#[test]
fn winner_claims_the_whole_pot() {
    let (mut litesvm, admin) = setup();
    let winner = finalized_single_winner(&mut litesvm, &admin);

    let pot = vault_balance(&litesvm);
    assert!(pot > 0);
    let before = litesvm.get_balance(&winner.pubkey()).unwrap();

    claim(&mut litesvm, &winner).assert_ok();

    let after = litesvm.get_balance(&winner.pubkey()).unwrap();
    assert!(after > before, "winner received the pot");
    let remaining = vault_balance(&litesvm);
    assert!(remaining > 0 && remaining < pot, "vault swept down to its rent floor");
}

#[test]
fn non_winner_cannot_claim() {
    let (mut litesvm, admin) = setup();
    let _winner = finalized_single_winner(&mut litesvm, &admin);

    let stranger = funded_keypair(&mut litesvm);
    claim(&mut litesvm, &stranger).assert_err(WorldCupError::NotWinner);
}

#[test]
fn claim_is_repeatable() {
    let (mut litesvm, admin) = setup();
    let winner = finalized_single_winner(&mut litesvm, &admin);

    claim(&mut litesvm, &winner).assert_ok();
    let after_first = litesvm.get_balance(&winner.pubkey()).unwrap();
    // Vault is already swept to its floor; a second claim sweeps ~nothing but still succeeds.
    claim(&mut litesvm, &winner).assert_ok();
    let after_second = litesvm.get_balance(&winner.pubkey()).unwrap();
    assert!(after_second <= after_first, "second claim pays no additional pot");
}

#[test]
fn counterfeit_config_is_rejected() {
    let (mut litesvm, admin) = setup();
    let _winner = finalized_single_winner(&mut litesvm, &admin);
    let pot_before = vault_balance(&litesvm);
    assert!(pot_before > 0);

    let attacker = funded_keypair(&mut litesvm);
    let fake_config = Keypair::new();
    let mut data = vec![0u8; Config::LEN];
    data[2] = 2; // wire offset: state = TournamentState::Finalized
    data[71..103].copy_from_slice(&attacker.pubkey().to_bytes()); // wire offset: winner
    litesvm
        .set_account(
            fake_config.pubkey(),
            Account { lamports: 1_000_000, data, owner: SYSTEM_PROGRAM_ID, executable: false, rent_epoch: 0 },
        )
        .unwrap();

    let (vault, _) = get_vault_pda();
    let accounts = vec![
        AccountMeta::new(attacker.pubkey(), true),
        AccountMeta::new(fake_config.pubkey(), false),
        AccountMeta::new(vault, false),
        AccountMeta::new_readonly(EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PROGRAM_ID, false),
    ];
    let ix = Instruction { program_id: PROGRAM_ID, accounts, data: vec![*crate::claim::DISCRIMINATOR] };
    build_and_send(&mut litesvm, &[&attacker], &attacker.pubkey(), &ix).assert_err(WorldCupError::NotProgramOwned);

    assert_eq!(vault_balance(&litesvm), pot_before, "pot untouched");
}

#[test]
fn cannot_claim_before_finalize() {
    let (mut litesvm, admin) = setup();
    init_config(&mut litesvm, &admin, LOCK_TS).assert_ok();

    let entrant = funded_keypair(&mut litesvm);
    submit_bracket(&mut litesvm, &entrant, &chalk_bracket(), 87).0.assert_ok();
    set_unix_timestamp(&mut litesvm, LOCK_TS);
    lock(&mut litesvm, &admin).assert_ok();
    post_full_chalk_oracle(&mut litesvm, &admin, 87);

    claim(&mut litesvm, &entrant).assert_err(WorldCupError::InvalidState);
}
