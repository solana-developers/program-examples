use transfer_sol_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, BanksClientError, ProgramTest};
use solana_sdk::{
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    transaction::Transaction,
    native_token::LAMPORTS_PER_SOL,
    system_instruction,
};
use std::{env, error::Error};

fn load_keypair() -> Result<Keypair, Box<dyn Error>> {
    // Try to get keypair path from environment variable
    let keypair_path = env::var("KEYPAIR_PATH")
        .unwrap_or_else(|_| format!("{}/.config/solana/id.json", env::var("HOME").unwrap()));
    println!("🔑 Loading keypair from: {}", keypair_path);
    Ok(read_keypair_file(&keypair_path)
        .map_err(|e| format!("Failed to load keypair: {}", e))?)
}

async fn setup() -> (BanksClient, Keypair, Hash) {
    println!("🚀 Setting up test environment...");
    
    let mut program_test = ProgramTest::new(
        "transfer_sol_program",
        transfer_sol_api::ID,
        processor!(transfer_sol_program::process_instruction),
    );
    
    let payer = load_keypair().expect("Failed to load keypair");
    println!("✅ Loaded payer wallet: {}", payer.pubkey());
    
    // Add initial balance for testing
    program_test.add_account(
        payer.pubkey(),
        solana_sdk::account::Account {
            lamports: 100 * LAMPORTS_PER_SOL,
            owner: solana_sdk::system_program::ID,
            executable: false,
            rent_epoch: 0,
            data: vec![],
        },
    );
    println!("💰 Added initial balance of {} SOL to payer", 100);
    
    program_test.prefer_bpf(true);
    println!("🔄 Starting program test...");
    let (banks, payer, hash) = program_test.start().await;
    println!("✅ Test environment setup complete");
    
    (banks, payer, hash)
}

fn lamports_to_sol_str(lamports: u64) -> String {
    let sols = lamports / LAMPORTS_PER_SOL;
    let remainder = lamports % LAMPORTS_PER_SOL;
    if remainder == 0 {
        format!("{} SOL", sols)
    } else {
        format!("{} SOL + {} lamports", sols, remainder)
    }
}

async fn get_new_blockhash(
    banks: &mut BanksClient,
    old_blockhash: Hash,
) -> Result<Hash, BanksClientError> {
    println!("🔄 Getting new blockhash...");
    let mut blockhash = old_blockhash;
    while blockhash == old_blockhash {
        std::thread::sleep(std::time::Duration::from_millis(400));
        blockhash = banks.get_latest_blockhash().await?;
    }
    println!("✅ New blockhash obtained");
    Ok(blockhash)
}

#[tokio::test]
async fn test_transfer_sol_with_cpi_success() {
    println!("\n🧪 Starting CPI transfer test");
    
    let (mut banks, payer, blockhash) = setup().await;
    let recipient = Keypair::new();
    println!("👤 Created recipient: {}", recipient.pubkey());
    
    // Get initial balances
    let initial_payer_balance = banks.get_balance(payer.pubkey()).await.unwrap();
    let initial_recipient_balance = banks.get_balance(recipient.pubkey()).await.unwrap();
    println!("💰 Initial payer balance: {}", lamports_to_sol_str(initial_payer_balance));
    println!("💰 Initial recipient balance: {}", lamports_to_sol_str(initial_recipient_balance));
    
    // Create and send transaction
    let transfer_amount = LAMPORTS_PER_SOL;
    println!("📝 Creating transfer instruction for {}", lamports_to_sol_str(transfer_amount));
    
    let ix = transfer_sol_with_cpi(
        payer.pubkey(),
        recipient.pubkey(),
        transfer_amount,
    );
    
    println!("🔑 Creating and signing transaction...");
    let tx = Transaction::new_signed_with_payer(
        &[ix.clone()],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    
    println!("📡 Sending transaction...");
    let result = banks.process_transaction(tx).await;
    assert!(result.is_ok(), "❌ Transaction failed: {:?}", result);
    println!("✅ Transaction successful");
    
    // Verify balances
    let final_payer_balance = banks.get_balance(payer.pubkey()).await.unwrap();
    let final_recipient_balance = banks.get_balance(recipient.pubkey()).await.unwrap();
    let fee = banks.get_fee_for_message(solana_sdk::message::Message::new(
        &[ix.clone()],
        Some(&payer.pubkey()),
    )).await.unwrap();
    
    println!("💰 Final payer balance: {}", lamports_to_sol_str(final_payer_balance));
    println!("💰 Final recipient balance: {}", lamports_to_sol_str(final_recipient_balance));
    println!("💸 Transaction fee: {:?} lamports", fee);
    
    assert_eq!(
        final_payer_balance,
        initial_payer_balance - transfer_amount - fee.unwrap(),
        "Incorrect payer balance"
    );
    assert_eq!(
        final_recipient_balance,
        initial_recipient_balance + transfer_amount,
        "Incorrect recipient balance"
    );
    println!("✅ CPI transfer test completed successfully\n");
}

#[tokio::test]
async fn test_transfer_sol_with_program_success() {
    println!("\n🧪 Starting program transfer test");
    
    let (mut banks, payer, mut blockhash) = setup().await;
    let recipient = Keypair::new();
    let program_owned_account = Keypair::new();
    println!("👤 Created recipient: {}", recipient.pubkey());
    println!("🏦 Created program-owned account: {}", program_owned_account.pubkey());
    
    // Create program-owned account with 2 SOL
    println!("📝 Creating program-owned account...");
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &program_owned_account.pubkey(),
        2 * LAMPORTS_PER_SOL,
        0,
        &transfer_sol_api::ID,
    );
    
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &program_owned_account],
        blockhash,
    );
    
    println!("📡 Sending account creation transaction...");
    banks.process_transaction(tx).await.unwrap();
    println!("✅ Program-owned account created successfully");
    
    blockhash = get_new_blockhash(&mut banks, blockhash).await.unwrap();
    
    // Get initial balances
    let initial_account_balance = banks.get_balance(program_owned_account.pubkey()).await.unwrap();
    let initial_recipient_balance = banks.get_balance(recipient.pubkey()).await.unwrap();
    
    println!("💰 Initial program-owned account balance: {}", 
        lamports_to_sol_str(initial_account_balance));
    
    // Create and send transfer transaction
    let transfer_amount = LAMPORTS_PER_SOL;
    println!("📝 Creating transfer instruction for {}", 
        lamports_to_sol_str(transfer_amount));
    
    let ix = transfer_sol_with_program(
        program_owned_account.pubkey(),
        recipient.pubkey(),
        transfer_amount,
    );
    
    println!("🔑 Creating and signing transaction...");
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    
    println!("📡 Sending transfer transaction...");
    let result = banks.process_transaction(tx).await;
    assert!(result.is_ok(), "❌ Transaction failed: {:?}", result);
    println!("✅ Transfer transaction successful");
    
    // Verify balances
    let final_account_balance = banks.get_balance(program_owned_account.pubkey()).await.unwrap();
    let final_recipient_balance = banks.get_balance(recipient.pubkey()).await.unwrap();
    
    println!("💰 Final program-owned account balance: {}", 
        lamports_to_sol_str(final_account_balance));
    println!("💰 Final recipient balance: {}", 
        lamports_to_sol_str(final_recipient_balance));
    
    assert_eq!(
        final_account_balance,
        initial_account_balance - transfer_amount,
        "Incorrect program-owned account balance"
    );
    assert_eq!(
        final_recipient_balance,
        initial_recipient_balance + transfer_amount,
        "Incorrect recipient balance"
    );
    println!("✅ Program transfer test completed successfully\n");
}