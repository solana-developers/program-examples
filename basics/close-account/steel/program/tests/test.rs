use close_account_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "close-account",
        close_account_api::ID,
        processor!(close_account_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    let name = "foobarbaz";
    let account = User::new(name).unwrap();

    // Submit initialize transaction.
    let ix = close_account_api::sdk::create_account(payer.pubkey(), CreateAccount(account.name));
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let user_pda = User::pda(payer.pubkey()).0;
    let pda_account = banks.get_account(user_pda).await.unwrap().unwrap();
    let name_deser = User::try_from_bytes(&pda_account.data).unwrap();

    assert_eq!(pda_account.owner, close_account_api::ID);
    assert_eq!(name_deser.to_string().unwrap().as_str(), name);

    // Test Closing an Account
    // Submit initialize transaction.
    let ix = close_account_api::sdk::close_account(payer.pubkey());
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let pda_account = banks.get_account(user_pda).await.unwrap();

    assert!(pda_account.is_none());
}
