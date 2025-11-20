use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::instruction::{mint_to, transfer_checked};
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

    //create token accounts
    let create_token_acc_ix1 = create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );
    let create_token_acc_ix2 = create_associated_token_account(
        &payer.pubkey(),
        &receiver.pubkey(),
        &mint.pubkey(),
        &spl_token_2022::ID,
    );

    // Derive ATAs
    let payer_ata = get_associated_token_address_with_program_id(
        &payer.pubkey(),
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
        &payer_ata,
        &payer.pubkey(),
        &[],
        1_000_000_000,
    )
    .unwrap();

    let transfer_ix = transfer_checked(
        &spl_token_2022::ID,
        &payer_ata,
        &mint.pubkey(),
        &receiver_ata,
        &payer.pubkey(),
        &[],
        1_000_000,
        6,
    )
    .unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[
            create_token_acc_ix1,
            create_token_acc_ix2,
            mint_ix,
            transfer_ix,
        ],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let res = banks.process_transaction(tx).await;
    assert!(res.is_err(), "Transfer unexpectedly succeeded");

    let err_string = format!("{:?}", res);

    // Custom(0x25) = Custom(37) = TokenError::NonTransferable
    assert!(
        err_string.contains("Custom(37)"),
        "Expected TokenError::NonTransferable (37), got: {}",
        err_string
    );
}
