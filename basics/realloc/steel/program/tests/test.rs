use realloc_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "realloc",
        realloc_api::ID,
        processor!(realloc_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn test_initialize_message() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Create message account
    let message_account = Keypair::new();
    let test_message = "Hello, Solana!".to_string();

    // Submit initialize transaction
    let ix = initialize(
        payer.pubkey(),
        message_account.pubkey(),
        test_message.clone(),
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &message_account],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify message was initialized correctly
    let account = banks.get_account(message_account.pubkey()).await.unwrap().unwrap();
    let message = Message::try_from_bytes(&account.data).unwrap();
    assert_eq!(message.message_len as usize, test_message.len());
    assert_eq!(
        String::from_utf8(message.message[..message.message_len as usize].to_vec()).unwrap(),
        test_message
    );
}

#[tokio::test]
async fn test_update_expand_message() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Create message account and initialize with short message
    let message_account = Keypair::new();
    let initial_message = "Short".to_string();
    
    let ix = initialize(
        payer.pubkey(),
        message_account.pubkey(),
        initial_message,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &message_account],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Update with longer message
    let longer_message = "This is a much longer message that will require reallocation".to_string();
    let ix = update(
        payer.pubkey(),
        message_account.pubkey(),
        longer_message.clone(),
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify message was updated correctly
    let account = banks.get_account(message_account.pubkey()).await.unwrap().unwrap();
    let message = Message::try_from_bytes(&account.data).unwrap();
    assert_eq!(message.message_len as usize, longer_message.len());
    assert_eq!(
        String::from_utf8(message.message[..message.message_len as usize].to_vec()).unwrap(),
        longer_message
    );
}

#[tokio::test]
async fn test_update_shrink_message() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Create message account and initialize with long message
    let message_account = Keypair::new();
    let initial_message = "This is a long message that will be shrunk".to_string();
    
    let ix = initialize(
        payer.pubkey(),
        message_account.pubkey(),
        initial_message,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &message_account],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Get initial balance
    let initial_balance = banks
        .get_account(message_account.pubkey())
        .await
        .unwrap()
        .unwrap()
        .lamports;

    // Update with shorter message
    let shorter_message = "Short msg".to_string();
    let ix = update(
        payer.pubkey(),
        message_account.pubkey(),
        shorter_message.clone(),
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify message was updated and rent was returned
    let account = banks.get_account(message_account.pubkey()).await.unwrap().unwrap();
    let message = Message::try_from_bytes(&account.data).unwrap();
    assert_eq!(message.message_len as usize, shorter_message.len());
    assert_eq!(
        String::from_utf8(message.message[..message.message_len as usize].to_vec()).unwrap(),
        shorter_message
    );
    assert!(account.lamports < initial_balance);
}

#[tokio::test]
async fn test_invalid_update() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Try to update non-existent account
    let message_account = Keypair::new();
    let test_message = "This should fail".to_string();
    
    let ix = update(
        payer.pubkey(),
        message_account.pubkey(),
        test_message,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_err());
}