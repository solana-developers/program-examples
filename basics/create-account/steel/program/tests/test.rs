use create_account_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "create_account_program",
        create_account_api::ID,
        processor!(create_account_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let new_account = Keypair::new();
    // Submit initialize transaction.
    let ix = create_system_account(payer.pubkey(), new_account.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &new_account],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify counter was initialized.
    let new_account_data = banks
        .get_account(new_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert!(new_account_data.owner == system_program::ID);
}
