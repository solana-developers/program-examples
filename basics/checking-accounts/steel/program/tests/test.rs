use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
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

    let account_to_create = Keypair::new();
    let account_to_change = Keypair::new();

    let account_to_change_ix = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &account_to_change.pubkey(),
        solana_sdk::native_token::LAMPORTS_PER_SOL,
        0,
        &steel_api::ID,
    );

    let tx = Transaction::new_signed_with_payer(
        &[account_to_change_ix],
        Some(&payer.pubkey()),
        &[&payer, &account_to_change],
        blockhash,
    );

    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Submit check_accounts transaction.
    let ix = check_accounts(
        payer.pubkey(),
        account_to_create.pubkey(),
        account_to_change.pubkey(),
    );
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
