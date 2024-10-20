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

    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // check the logs to see if the program executed correctly
}
