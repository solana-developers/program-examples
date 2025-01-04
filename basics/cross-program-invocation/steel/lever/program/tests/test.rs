use lever_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "lever_program",
        lever_api::ID,
        processor!(lever_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
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

    // Verify power account was initialized.
    let power_address = power_account.pubkey();
    let power_account = banks.get_account(power_address).await.unwrap().unwrap();
    let power_status = PowerStatus::try_from_bytes(&power_account.data).unwrap();
    assert_eq!(power_account.owner, lever_api::ID);
    assert_eq!(power_status.is_on, 0);

    // Submit switch_power transaction.
    let ix = switch_power(power_address, "Chris");
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // // Verify power_status was changed.
    let power_account = banks.get_account(power_address).await.unwrap().unwrap();
    let power_status = PowerStatus::try_from_bytes(&power_account.data).unwrap();
    assert_eq!(power_account.owner, lever_api::ID);
    assert_eq!(power_status.is_on, 1);
}
