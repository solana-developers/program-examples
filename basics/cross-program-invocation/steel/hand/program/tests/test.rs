use hand_api::prelude::*;
use lever_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "hand_program",
        hand_api::ID,
        processor!(hand_program::process_instruction),
    );

    program_test.add_program(
        "lever_program",
        lever_api::ID,
        processor!(lever_program::process_instruction),
    );

    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let power_account = Keypair::new();

    // Submit initialize transaction.
    let ix = initialize(payer.pubkey(), power_account.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &power_account],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Submit pull_lever transaction.
    let ix = pull_lever(power_account.pubkey(), "Chris");
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let ix = pull_lever(power_account.pubkey(), "Ashley");
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
