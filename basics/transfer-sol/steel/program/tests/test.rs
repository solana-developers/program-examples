use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer, transaction::Transaction,
};
use transfer_sol_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "transfer_sol_program",
        transfer_sol_api::ID,
        processor!(transfer_sol_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test_transfer_sol_with_cpi() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let recipient = Keypair::new();
    let amount = 1000_000_000u64;

    // Submit transfer_sol_with_cpi transaction.
    let ix = transfer_sol_with_cpi(payer.pubkey(), recipient.pubkey(), amount);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // check if the recipient account has the correct amount of lamports
    let recipient_account_balance = banks.get_balance(recipient.pubkey()).await.unwrap();

    assert_eq!(recipient_account_balance, amount);
}

#[tokio::test]
async fn run_test_transfer_sol_with_program() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let recipient = Keypair::new();
    let program_owned_account = Keypair::new();
    let amount = 1000_000_000u64;

    let create_program_owned_account_ix = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &program_owned_account.pubkey(),
        1 * LAMPORTS_PER_SOL,
        0,
        &transfer_sol_api::ID,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_program_owned_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &program_owned_account],
        blockhash,
    );

    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Submit transfer_sol_with_program transaction.
    let ix = transfer_sol_with_program(program_owned_account.pubkey(), recipient.pubkey(), amount);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // check if the recipient account has the correct amount of lamports
    let recipient_account_balance = banks.get_balance(recipient.pubkey()).await.unwrap();
    assert_eq!(recipient_account_balance, amount);

    let program_owned_account_balance = banks
        .get_balance(program_owned_account.pubkey())
        .await
        .unwrap();

    assert_eq!(program_owned_account_balance, 0);
}
