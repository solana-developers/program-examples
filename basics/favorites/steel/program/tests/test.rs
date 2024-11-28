use favorites_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "favorites",
        favorites_api::ID,
        processor!(favorites_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn test_initialize_favorites() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Test data
    let number = 42;
    let color = "blue".to_string();
    let hobbies = vec!["reading".to_string(), "coding".to_string()];

    // Create set_favorites instruction
    let ix = set_favorites(
        payer.pubkey(),
        number,
        color.clone(),
        hobbies.clone(),
    ).unwrap();

    // Submit transaction
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let result = banks.process_transaction(tx).await;
    assert!(result.is_ok());

    // Verify account data
    let favorites_account = banks
        .get_account(favorites_pda(&payer.pubkey()).0)
        .await
        .unwrap()
        .unwrap();

    let favorites = Favorites::try_from_bytes(&favorites_account.data).unwrap();
    let (stored_number, stored_color, stored_hobbies) = decode_favorites(&favorites);

    assert_eq!(stored_number, number);
    assert_eq!(stored_color, color);
    assert_eq!(stored_hobbies, hobbies);
}

#[tokio::test]
async fn test_update_favorites() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Initial data
    let initial_number = 42;
    let initial_color = "blue".to_string();
    let initial_hobbies = vec!["reading".to_string()];

    // Initialize favorites first
    let ix = set_favorites(
        payer.pubkey(),
        initial_number,
        initial_color,
        initial_hobbies,
    ).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    banks.process_transaction(tx).await.unwrap();

    // Update with new data
    let new_number = 100;
    let new_color = "green".to_string();
    let new_hobbies = vec!["gaming".to_string(), "cooking".to_string()];

    let ix = set_favorites(
        payer.pubkey(),
        new_number,
        new_color.clone(),
        new_hobbies.clone(),
    ).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let result = banks.process_transaction(tx).await;
    assert!(result.is_ok());

    // Verify updated data
    let favorites_account = banks
        .get_account(favorites_pda(&payer.pubkey()).0)
        .await
        .unwrap()
        .unwrap();

    let favorites = Favorites::try_from_bytes(&favorites_account.data).unwrap();
    let (stored_number, stored_color, stored_hobbies) = decode_favorites(&favorites);

    assert_eq!(stored_number, new_number);
    assert_eq!(stored_color, new_color);
    assert_eq!(stored_hobbies, new_hobbies);
}

#[tokio::test]
async fn test_validation_limits() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Test string too long
    let long_color = "x".repeat(STRING_MAX_SIZE + 1);
    let ix = set_favorites(
        payer.pubkey(),
        42,
        long_color,
        vec!["hobby".to_string()],
    );
    assert!(ix.is_err());

    // Test too many hobbies
    let too_many_hobbies = (0..MAX_HOBBIES + 1)
        .map(|i| format!("hobby{}", i))
        .collect();
    let ix = set_favorites(
        payer.pubkey(),
        42,
        "blue".to_string(),
        too_many_hobbies,
    );
    assert!(ix.is_err());

    // Test hobby string too long
    let long_hobby = "x".repeat(STRING_MAX_SIZE + 1);
    let ix = set_favorites(
        payer.pubkey(),
        42,
        "blue".to_string(),
        vec![long_hobby],
    );
    assert!(ix.is_err());
}

#[tokio::test]
async fn test_non_signer() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let non_signer = Keypair::new();

    // Try to set favorites with non-signing account
    let ix = set_favorites(
        non_signer.pubkey(),
        42,
        "blue".to_string(),
        vec!["hobby".to_string()],
    ).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer], // non_signer is not included
        blockhash,
    );

    let result = banks.process_transaction(tx).await;
    assert!(result.is_err());
}