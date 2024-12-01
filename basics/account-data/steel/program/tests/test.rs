use account_data_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "account_data_program",
        account_data_api::ID,
        processor!(account_data_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn creates_address_info_account() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    let address_info = AddressInfoData::new(
        "Alice".to_string(),
        42,
        "Wonderland".to_string(),
        "Solana Beach".to_string(),
    );

    // Submit create_address_info transaction.
    let ix = create_address_info(payer.pubkey(), address_info);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify address was initialized.
    let address = account_pda().0;
    let account = banks.get_account(address).await.unwrap().unwrap();
    let info = AddressInfo::try_from_bytes(&account.data).unwrap();
    assert_eq!(account.owner, account_data_api::ID);
    assert_eq!(info.data, address_info);
}
