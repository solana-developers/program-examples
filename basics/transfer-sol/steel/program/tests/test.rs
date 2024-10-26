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
async fn transfer_with_program_works() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // 1 SOL
    let amount = 1 * LAMPORTS_PER_SOL;

    // Generate a couple of keypairs to create accounts owned by our program
    let acc_1 = Keypair::new();
    let acc_2 = Keypair::new();

    // Create the program accounts
    let create_account_instruction = |pubkey| {
        solana_program::system_instruction::create_account(
            &payer.pubkey(),
            pubkey,
            amount,
            0,
            &transfer_sol_api::ID,
        )
    };

    let tx_create_accounts = Transaction::new_signed_with_payer(
        &[
            create_account_instruction(&acc_1.pubkey()),
            create_account_instruction(&acc_2.pubkey()),
        ],
        Some(&payer.pubkey()),
        &[&payer, &acc_1, &acc_2],
        blockhash,
    );

    let res = banks.process_transaction(tx_create_accounts).await;
    assert!(res.is_ok());

    // Let's transfer some lamports from acc_1 to acc_2
    let latest_blockhash = banks.get_latest_blockhash().await.unwrap();

    let ix = with_program(acc_1.pubkey(), acc_2.pubkey(), amount);
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Now, let's check the balances
    let acc_1_balance = banks.get_balance(acc_1.pubkey()).await.unwrap();
    assert_eq!(acc_1_balance, 0);
    let acc_2_balance = banks.get_balance(acc_2.pubkey()).await.unwrap();
    assert_eq!(acc_2_balance, 2 * amount);
}

#[tokio::test]
async fn transfer_with_cpi_works() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // 1 SOL
    let amount = 1 * LAMPORTS_PER_SOL;

    // Generate a new keypair for the recipient
    let recipient = Keypair::new();

    // Submit transfer with cpi transaction.
    let ix = with_cpi(payer.pubkey(), recipient.pubkey(), amount);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let balance = banks.get_balance(recipient.pubkey()).await.unwrap();
    assert_eq!(balance, amount);
}
