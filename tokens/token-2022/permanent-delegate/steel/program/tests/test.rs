use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::instruction::{burn_checked, mint_to};
use steel::*;
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
    let delegate = Keypair::new();

    // Submit initialize transaction.
    let ix = initialize(payer.pubkey(), mint.pubkey(), delegate.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //Create Token Account, Mint Tokens 
    //Burn as Permanent Delegate
    //Using a new/random keypair to experiment
    let receiver = Keypair::new();

    let create_token_acc_ix = create_associated_token_account(
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
    let mint_ix = mint_to(
        &spl_token_2022::ID,
        &mint.pubkey(),
        &receiver_ata,
        &payer.pubkey(),
        &[],
        1000,
    )
    .unwrap();

    let burn_ix = burn_checked(
        &spl_token_2022::ID,
        &receiver_ata,
        &mint.pubkey(),
        &delegate.pubkey(),
        &[],
        1000,
        6,
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_token_acc_ix, mint_ix, burn_ix],
        Some(&payer.pubkey()),
        &[&payer, &delegate],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
