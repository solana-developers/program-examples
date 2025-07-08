use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    program_pack::Pack, signature::Keypair, signer::Signer, system_instruction::create_account,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::{
    instruction::{initialize_account3, mint_to},
    state::Account,
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
    let receiver = Keypair::new();

    // Submit initialize transaction.
    let ix = initialize(payer.pubkey(), mint.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // //Create Ata and try out a transaction
    let create_token_acc_ix1 = create_associated_token_account(
        &payer.pubkey(),
        &receiver.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );

    let receiver_ata = get_associated_token_address_with_program_id(
        &receiver.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );

    //Mint Tokens
    let mint_ix = mint_to(
        &spl_token_2022::ID,
        &mint.pubkey(),
        &receiver_ata,
        &payer.pubkey(),
        &[],
        1_000_000_000,
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_token_acc_ix1, mint_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    
    assert!(res.is_err(), "MintTo unexpectedly succeeded");
    let err_string = format!("{:?}", res);

    assert!(
        err_string.contains("Custom(17)"),
        "Expected TokenError::AccountFrozen (17), got: {}",
        err_string
    );

    //Update State and Try to Mint Again
    let update_ix = update_default_account_state(payer.pubkey(), mint.pubkey());

    let new_token_acc = Keypair::new();
    let create_ix = create_account(
        &payer.pubkey(),
        &new_token_acc.pubkey(),
        banks
            .get_rent()
            .await
            .unwrap()
            .minimum_balance(Account::LEN),
        Account::LEN as u64,
        &spl_token_2022::ID,
    );
    let init_ix = initialize_account3(
        &spl_token_2022::ID,
        &new_token_acc.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
    )
    .unwrap();

    //Mint Tokens
    let mint_ix2 = mint_to(
        &spl_token_2022::ID,
        &mint.pubkey(),
        &new_token_acc.pubkey(),
        &payer.pubkey(),
        &[],
        1_000_000_000,
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[update_ix, create_ix, init_ix, mint_ix2],
        Some(&payer.pubkey()),
        &[&payer, &mint, &new_token_acc],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
