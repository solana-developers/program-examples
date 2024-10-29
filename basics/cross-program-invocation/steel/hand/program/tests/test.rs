use lever_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "hand",
        hand_api::ID,
        processor!(hand_program::process_instruction),
    );

    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn test_pull_lever() {
    let (mut banks, payer, recent_blockhash) = setup().await;
    
    // First initialize the lever program's power account
    let power_seed = b"power";
    let seeds: &[&[u8]] = &[power_seed];
    let (power_address, _) = Pubkey::find_program_address(seeds, &lever_api::ID);
    
    let init_ix = lever_api::sdk::initialize(
        payer.pubkey(),
        power_address,
    );
    
    let init_tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let result = banks.process_transaction(init_tx).await;
    assert!(result.is_ok(), "Failed to initialize power account");

    // Now test the hand program's pull_lever instruction
    let pull_lever_ix = hand_api::sdk::pull_lever(
        power_address,
        "test_user".to_string(),
    );

    let pull_tx = Transaction::new_signed_with_payer(
        &[pull_lever_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks.process_transaction(pull_tx).await;
    assert!(result.is_ok(), "Failed to pull lever");

    // Verify power state changed
    let power_account = banks.get_account(power_address).await.unwrap().unwrap();
    let power_status = PowerStatus::try_from_bytes(&power_account.data).unwrap();
    assert_eq!(power_status.is_on, 1, "Power should be on after pulling lever");
}

#[tokio::test]
async fn test_pull_lever_uninitialized_power() {
    let (mut banks, payer, recent_blockhash) = setup().await;
    
    // Try to pull lever without initializing power account
    let power_seed = b"power";
    let seeds: &[&[u8]] = &[power_seed];
    let (power_address, _) = Pubkey::find_program_address(seeds, &lever_api::ID);
    
    let pull_lever_ix = hand_api::sdk::pull_lever(
        power_address,
        "test_user".to_string(),
    );

    let pull_tx = Transaction::new_signed_with_payer(
        &[pull_lever_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks.process_transaction(pull_tx).await;
    assert!(result.is_err(), "Expected error when pulling uninitialized lever");
}

#[tokio::test]
async fn test_pull_lever_invalid_name() {
    let (mut banks, payer, recent_blockhash) = setup().await;
    
    // Initialize power account
    let power_seed = b"power";
    let seeds: &[&[u8]] = &[power_seed];
    let (power_address, _) = Pubkey::find_program_address(seeds, &lever_api::ID);
    
    let init_ix = lever_api::sdk::initialize(
        payer.pubkey(),
        power_address,
    );
    
    banks.process_transaction(Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    )).await.unwrap();
    
    // Try pulling lever with too long name
    let long_name = "this_name_is_way_too_long_and_should_cause_an_error".to_string();
    let pull_lever_ix = hand_api::sdk::pull_lever(
        power_address,
        long_name,
    );

    let pull_tx = Transaction::new_signed_with_payer(
        &[pull_lever_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks.process_transaction(pull_tx).await;
    assert!(result.is_err(), "Expected error with invalid name length");
}