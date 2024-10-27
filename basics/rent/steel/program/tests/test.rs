use rent_example_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "rent_example_program",
        rent_example_api::ID,
        processor!(rent_example_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn test_create_system_account() {
    // Setup test environment
    let (mut banks_client, payer, recent_blockhash) = setup().await;

    // Generate a new keypair for the account we'll create
    let new_account = Keypair::new();
    
    // Test data
    let name = "John Doe";
    let address = "123 Blockchain Street";

    // Create the instruction
    let ix = create_system_account(
        payer.pubkey(),
        new_account.pubkey(),
        name.to_string(),
        address.to_string(),
    );

    // Create and send transaction
    let mut transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &new_account],
        recent_blockhash,
    );

    // Process transaction
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "Failed to process transaction: {:?}", result);

    // Fetch and verify the created account
    let account = banks_client
        .get_account(new_account.pubkey())
        .await
        .expect("Failed to get account")
        .expect("Account not found");

    // Verify account owner
    assert_eq!(account.owner, rent_example_api::ID, "Incorrect account owner");

    // Deserialize and verify account data
    let address_account = Address::try_from_bytes(&account.data)
        .expect("Failed to deserialize account data");

    // Convert stored bytes back to strings for comparison
    let stored_name = String::from_utf8(
        address_account.name
            .iter()
            .take_while(|&&b| b != 0)
            .cloned()
            .collect::<Vec<u8>>()
    ).unwrap();

    let stored_address = String::from_utf8(
        address_account.address
            .iter()
            .take_while(|&&b| b != 0)
            .cloned()
            .collect::<Vec<u8>>()
    ).unwrap();

    // Verify the stored data matches what we sent
    assert_eq!(stored_name, name, "Stored name doesn't match");
    assert_eq!(stored_address, address, "Stored address doesn't match");
}