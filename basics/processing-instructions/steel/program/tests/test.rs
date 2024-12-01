use processing_instructions_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "processing_instructions_program",
        processing_instructions_api::ID,
        processor!(processing_instructions_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

// Helper function to assert the presence of log messages
fn assert_log_contains(logs: &[String], expected: &str) {
    assert!(
        logs.iter().any(|msg| msg.contains(expected)),
        "Missing log: {}",
        expected
    );
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Submit transaction.
    let ix1 = go_to_the_park(payer.pubkey(), GoToTheParkData::new("Jimmy".to_string(), 3));
    let ix2 = go_to_the_park(payer.pubkey(), GoToTheParkData::new("Mary".to_string(), 10));
    let tx = Transaction::new_signed_with_payer(
        &[ix1, ix2],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let res = banks.process_transaction_with_metadata(tx).await;
    assert!(res.is_ok(), "Process Transaction failed: {:?}", res.err());
    let tx_result = res.unwrap();
    assert!(
        tx_result.result.is_ok(),
        "Transaction failed: {:?}",
        tx_result.result.err()
    );

    let metadata = tx_result.metadata.unwrap();

    // check the logs
    // we got 2 instructions, we must see 2 consecutive logs
    // - Welcome to the park, {name}!
    // - You are NOT tall enough... and You are tall enough...
    assert_log_contains(&metadata.log_messages, "Welcome to the park, Jimmy!");
    assert_log_contains(&metadata.log_messages, "You are NOT tall enough");
    assert_log_contains(&metadata.log_messages, "Welcome to the park, Mary!");
    assert_log_contains(&metadata.log_messages, "You are tall enough");
}
