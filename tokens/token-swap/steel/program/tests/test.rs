use token_swap_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "token-swap",
        token_swap_api::ID,
        processor!(token_swap_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Submit initialize transaction.
    let ix = initialize(payer.pubkey());
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify counter was initialized.
    let counter_address = counter_pda().0;
    let counter_account = banks.get_account(counter_address).await.unwrap().unwrap();
    let counter = Counter::try_from_bytes(&counter_account.data).unwrap();
    assert_eq!(counter_account.owner, token_swap_api::ID);
    assert_eq!(counter.value, 0);

    // Submit add transaction.
    let ix = add(payer.pubkey(), 42);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify counter was incremented.
    let counter_account = banks.get_account(counter_address).await.unwrap().unwrap();
    let counter = Counter::try_from_bytes(&counter_account.data).unwrap();
    assert_eq!(counter.value, 42);
}

