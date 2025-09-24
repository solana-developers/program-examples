use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    program_pack::Pack, signature::Keypair, signer::Signer, system_instruction::create_account,
    transaction::Transaction,
};
use spl_token_2022::{
    instruction::{initialize_mint2, mint_to, set_authority, AuthorityType},
    state::Mint,
};
use steel_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "steel_program",
        steel_api::ID,
        processor!(steel_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    let mint = Keypair::new();
    let token_acc = Keypair::new();
    let receiver_token_acc = Keypair::new();

    let create_acc_ix = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        banks.get_rent().await.unwrap().minimum_balance(Mint::LEN),
        Mint::LEN as u64,
        &spl_token_2022::ID,
    );
    let init_mint_ix = initialize_mint2(
        &spl_token_2022::ID,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        6,
    )
    .unwrap();
    let mint_to_ix = mint_to(
        &spl_token_2022::ID,
        &mint.pubkey(),
        &token_acc.pubkey(),
        &payer.pubkey(),
        &[],
        1_000_000,
    )
    .unwrap();

    //Try to set authority
    let auth_ix = set_authority(
        &spl_token_2022::ID,
        &token_acc.pubkey(),
        Some(&receiver_token_acc.pubkey()),
        AuthorityType::AccountOwner,
        &payer.pubkey(),
        &[],
    )
    .unwrap();

    // Submit initialize transaction.
    let ix = initialize(payer.pubkey(), mint.pubkey(), token_acc.pubkey());
    let ix2 = initialize(payer.pubkey(), mint.pubkey(), receiver_token_acc.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[create_acc_ix, init_mint_ix, ix, ix2, mint_to_ix, auth_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint, &token_acc, &receiver_token_acc],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;

    let err_string = format!("{:?}", res);

    assert!(
        err_string.contains("Custom(34)"),
        "Expected TokenError::OwnerCannotBeChanged (34), got: {}",
        err_string
    );
}
