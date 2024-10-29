use lever_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "lever",
        lever_api::ID,
        processor!(lever_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

async fn get_power_status(banks: &mut BanksClient, power_address: Pubkey) -> PowerStatus {
    let account = banks.get_account(power_address).await.unwrap().unwrap();
    *PowerStatus::try_from_bytes(&account.data).unwrap()
}

#[tokio::test]
async fn test_initialize_power() {
    // Setup test environment
    let (mut banks, payer, recent_blockhash) = setup().await;
    
    // Calculate PDA for power account
    let power_seed = b"power";
    let seeds: &[&[u8]] = &[power_seed];
    let (power_address, _) = Pubkey::find_program_address(seeds, &lever_api::ID);
    
    // Create initialize instruction
    let ix = lever_api::sdk::initialize(
        payer.pubkey(),
        power_address,
    );
    
    // Create and submit transaction
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let result = banks.process_transaction(tx).await;
    assert!(result.is_ok(), "Failed to initialize power: {:?}", result);
    
    // Verify power account was created and initialized correctly
    let power_account = banks.get_account(power_address).await.unwrap().unwrap();
    assert_eq!(power_account.owner, lever_api::ID);
    
    let power_status = PowerStatus::try_from_bytes(&power_account.data).unwrap();
    assert_eq!(power_status.is_on, 0, "Power should be initialized as off");
}

#[tokio::test]
async fn test_switch_power() {
    let (mut banks, payer, recent_blockhash) = setup().await;
    
    // Initialize power account first
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
    
    banks.process_transaction(init_tx).await.unwrap();
    
    // Test switching power on
    let switch_ix = lever_api::sdk::switch_power(
        power_address,
        "test_user".to_string(),
    );
    
    let switch_tx = Transaction::new_signed_with_payer(
        &[switch_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let result = banks.process_transaction(switch_tx).await;
    assert!(result.is_ok(), "Failed to switch power: {:?}", result);
    
    // Verify power was switched on
    let power_status = get_power_status(&mut banks, power_address).await;
    assert_eq!(power_status.is_on, 1, "Power should be on after first switch");
    
    // Test switching power off
    let switch_ix = lever_api::sdk::switch_power(
        power_address,
        "test_user".to_string(),
    );
    
    let switch_tx = Transaction::new_signed_with_payer(
        &[switch_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    banks.process_transaction(switch_tx).await.unwrap();
    
    // Verify power was switched off
    let power_status = get_power_status(&mut banks, power_address).await;
    assert_eq!(power_status.is_on, 0, "Power should be off after second switch");
}

#[tokio::test]
async fn test_switch_power_invalid_name() {
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
    
    // Try switching with too long name
    let long_name = "this_name_is_way_too_long_and_should_cause_an_error".to_string();
    let switch_ix = lever_api::sdk::switch_power(
        power_address,
        long_name,
    );
    
    let switch_tx = Transaction::new_signed_with_payer(
        &[switch_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let result = banks.process_transaction(switch_tx).await;
    assert!(result.is_err(), "Expected error for too long name");
}

#[tokio::test]
async fn test_switch_power_uninitialized() {
    let (mut banks, payer, recent_blockhash) = setup().await;
    
    // Try to switch power without initializing
    let power_seed = b"power";
    let seeds: &[&[u8]] = &[power_seed];
    let (power_address, _) = Pubkey::find_program_address(seeds, &lever_api::ID);
    
    let switch_ix = lever_api::sdk::switch_power(
        power_address,
        "test_user".to_string(),
    );
    
    let switch_tx = Transaction::new_signed_with_payer(
        &[switch_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let result = banks.process_transaction(switch_tx).await;
    assert!(result.is_err(), "Expected error for uninitialized power account");
}