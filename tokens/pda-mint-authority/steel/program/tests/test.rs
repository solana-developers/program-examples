/*
use api::prelude::*;
use solana_program::{hash::Hash, program_pack::Pack};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, compute_budget::ComputeBudgetInstruction, signature::Keypair, signer::Signer, transaction::Transaction};
use pda_mint_authority_program::process_instruction;
use spl_token::state::Mint;
use solana_program::program_option::COption;
use mpl_token_metadata::accounts::Metadata;
use steel::*;
use solana_client::rpc_client::RpcClient;

// Helper function to get PDA addresses
fn get_addresses(_mint_seed: &[u8]) -> (Pubkey, Pubkey, u8) {
    let (mint_pda, bump) = Pubkey::find_program_address(
        &[MintAuthorityPda::SEED_PREFIX.as_bytes()],  
        &api::ID
    );
    
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint_pda.as_ref()
        ],
        &mpl_token_metadata::ID
    );
    
    (mint_pda, metadata_pda, bump)
}

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "pda_mint_authority_program",
        api::ID,
        processor!(process_instruction),
    );

    // Add Token Program
    program_test.add_account(
        spl_token::id(),
        Account {
            lamports: 1_000_000_000,
            data: vec![],
            owner: solana_program::bpf_loader_upgradeable::id(), // Use upgradeable loader
            executable: true,
            rent_epoch: 0,
        },
    );

    // Add Token Metadata Program
    program_test.add_account(
        mpl_token_metadata::ID,
        Account {
            lamports: 1_000_000_000,
            data: vec![],
            owner: solana_program::bpf_loader_upgradeable::id(), // Use upgradeable loader
            executable: true,
            rent_epoch: 0,
        },
    );

    program_test.prefer_bpf(true);
    
    program_test.start().await
}


async fn verify_program_deployment(banks_client: &mut BanksClient, program_id: Pubkey) {
    if let Ok(Some(program)) = banks_client.get_account(program_id).await {
        println!("Program found: {:?}", program_id);
        println!("Program owner: {:?}", program.owner);
        println!("Program executable: {}", program.executable);
    } else {
        eprintln!("Program not found: {:?}", program_id);
    }
}

async fn debug_transaction_accounts(banks: &mut BanksClient, tx: &Transaction) {
    println!("\n=== Transaction Debug Info ===");
    println!("Number of signatures: {}", tx.signatures.len());
    println!("Signing pubkeys: {:?}", tx.message.header.num_required_signatures);
    println!("Readonly signers: {:?}", tx.message.header.num_readonly_signed_accounts);
    println!("Readonly non-signers: {:?}", tx.message.header.num_readonly_unsigned_accounts);
    
    // Print account info for each account in transaction
    for (i, acc) in tx.message.account_keys.iter().enumerate() {
        if let Ok(Some(account)) = banks.get_account(*acc).await {
            println!("\nAccount #{}: {:?}", i, acc);
            println!("  Owner: {:?}", account.owner);
            println!("  Lamports: {}", account.lamports);
            println!("  Executable: {}", account.executable);
            println!("  Data len: {}", account.data.len());
            println!("  Is signer: {}", tx.message.is_signer(i));
            println!("  Is writable: {}", tx.message.is_writable(i));
        } else {
            println!("\nAccount #{}: {:?} (not found - will be created)", i, acc);
        }
    }
    println!("\nInstruction Data:");
    for (i, inst) in tx.message.instructions.iter().enumerate() {
        println!("Instruction #{}", i);
        println!("  Program ID: {:?}", tx.message.account_keys[inst.program_id_index as usize]);
        println!("  Input accounts: {:?}", inst.accounts);
        println!("  Data: {:?}", inst.data);
    }
    println!("===========================\n");
}

#[tokio::test]
async fn test_create_token() {
    // First, let's set up our test environment with a fake Solana network
    let (mut banks, payer, recent_blockhash) = setup().await;

    // Verify token program exists
    println!("\n=== Token Program Verification ===");
    if let Ok(Some(token_program)) = banks.get_account(spl_token::id()).await {
        println!("Token Program Details:");
        println!("  Owner: {}", token_program.owner);
        println!("  Executable: {}", token_program.executable);
        println!("  Data length: {}", token_program.data.len());
        println!("  Lamports: {}", token_program.lamports);
    } else {
        println!("Token Program not found!");
    }

    // Let's make sure all our programs are properly deployed and ready to go
    println!("Verifying your program...");
    verify_program_deployment(&mut banks, api::ID).await;

    // We need the Token Metadata program for NFT stuff
    println!("Verifying Token Metadata program...");
    verify_program_deployment(&mut banks, mpl_token_metadata::ID).await;

    // And of course, the standard SPL Token program
    println!("Verifying SPL Token program...");
    verify_program_deployment(&mut banks, spl_token::id()).await;
    
    // Set up some basic info for our test token
    let token_name = "Test".to_string();
    let token_symbol = "TST".to_string();
    let token_uri = "https://test.json".to_string();  // This could be metadata JSON in a real token
    
    // Generate our PDAs - these are special addresses that our program controls
    let (mint_pda, metadata_pda, bump) = get_addresses(MintAuthorityPda::SEED_PREFIX.as_bytes());

    println!("\n=== PDA Info ===");
    println!("Mint PDA: {}", mint_pda);
    println!("Metadata PDA: {}", metadata_pda);
    println!("Bump: {}", bump);

    // Create the instruction that will set up our new token
    let create_token_ix = create_token(
        payer.pubkey(),
        token_name.clone(),
        token_symbol.clone(),
        token_uri.clone()
    );

    println!("Accounts in instruction:");
    println!("1. Payer: {}", payer.pubkey());
    println!("2. Mint Account: {}", mint_pda);
    println!("3. Mint Authority: {}", mint_pda);
    println!("4. Metadata Account: {}", metadata_pda);

    println!("\n=== Transaction Signing Info ===");
    println!("Payer (signer): {}", payer.pubkey());
    println!("Mint Authority (PDA, not a signer): {}", mint_pda);

    let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(400_000);  // Increased from default 200k
    let set_price_ix = ComputeBudgetInstruction::set_compute_unit_price(1);
    
    // Build and send our transaction - only need payer to sign since PDAs are handled inside the program
    let tx = Transaction::new_signed_with_payer(
        &[        
        compute_budget_ix,
        set_price_ix,
        create_token_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );


    // Add detailed debugging before processing
    debug_transaction_accounts(&mut banks, &tx).await;

    // Try to process the transaction and catch any errors
    let result = banks.process_transaction(tx.clone()).await;

    println!("\n=== Signer Analysis ===");
    println!("Transaction requires {} signatures", tx.message.header.num_required_signatures);
    println!("Actual signer: {}", payer.pubkey());
    println!("Expected mint authority (PDA): {}", mint_pda);

    // If something goes wrong, let's get detailed error info to help debug
    if let Err(err) = &result {
        
        println!("Transaction failed with error: {:?}", err);

        // Get transaction status and logs - super helpful for debugging
        match banks.get_transaction_status(tx.signatures[0]).await {
            Ok(Some(tx_status)) => {
                println!("Transaction status: {:?}", tx_status.confirmation_status);
                if let Some(logs) = tx_status.err {
                    println!("Transaction logs: {}", logs);
                } else {
                    eprintln!("No logs available for this transaction.");
                }
            }
            Ok(None) => println!("Transaction not found or not yet confirmed."),
            Err(fetch_err) => eprintln!("Failed to get transaction status: {:?}", fetch_err),
        }
    }
    
    // Make sure our transaction succeeded
    assert!(result.is_ok(), "Failed to process transaction: {:?}", result.err());

    // Now let's check if our mint account was created correctly
    let mint_account = banks.get_account(mint_pda).await.unwrap().unwrap();
    assert_eq!(mint_account.owner, spl_token::ID, "Mint account should be owned by Token program");
    
    // Parse the mint account data and verify everything looks good
    let mint_data = Mint::unpack(&mint_account.data).unwrap();
    assert_eq!(mint_data.decimals, 9, "We should have 9 decimals");
    assert!(mint_data.is_initialized, "Mint should be initialized");
    assert_eq!(
        mint_data.mint_authority,
        COption::Some(mint_pda),
        "Our PDA should be the mint authority"
    );
    
    // Now for the fun part - let's try minting some tokens!
    let mint_amount = 1_000_000;  // Let's mint a million tokens
    
    // Figure out where we're going to send these tokens
    let token_account = spl_associated_token_account::get_associated_token_address(
        &payer.pubkey(),
        &mint_pda
    );

    // First we need to create a token account to hold our new tokens
    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),  // payer
        &payer.pubkey(),  // wallet that will own the tokens
        &mint_pda,        // which token type
        &spl_token::id(), // token program
    );

    // Now create the instruction to mint our tokens
    let mint_tokens_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_pda,       // mint account
        &token_account,  // where to send the tokens
        &mint_pda,       // PDA as mint authority
        &[],            // no extra signers needed
        mint_amount,    // how many tokens to mint
    ).unwrap();

    // Bundle it all together in a transaction
    let mint_transaction = Transaction::new_signed_with_payer(
        &[create_ata_ix, mint_tokens_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // Send it off!
    let mint_result = banks.process_transaction(mint_transaction).await;
    assert!(mint_result.is_ok(), "Failed to mint tokens: {:?}", mint_result.err());

    // Make sure our tokens arrived
    let token_account_info = banks.get_account(token_account).await.unwrap().unwrap();
    let token_account_data = spl_token::state::Account::unpack(&token_account_info.data).unwrap();
    assert_eq!(token_account_data.amount, mint_amount, "We should have exactly the amount we minted");
    
    // Check if our metadata got created properly
    let metadata_account = banks.get_account(metadata_pda).await.unwrap().unwrap();
    assert_eq!(
        metadata_account.owner,
        mpl_token_metadata::ID,
        "Metadata account should be owned by Token Metadata program"
    );
    
    // Parse and verify all our metadata
    let metadata = Metadata::safe_deserialize(&metadata_account.data).expect("Failed to deserialize metadata");
    assert_eq!(metadata.name.trim_matches(char::from(0)), token_name, "Name should match");
    assert_eq!(metadata.symbol.trim_matches(char::from(0)), token_symbol, "Symbol should match");
    assert_eq!(metadata.uri.trim_matches(char::from(0)), token_uri, "URI should match");
    assert_eq!(metadata.mint, mint_pda, "Metadata should point to our mint");
    assert_eq!(metadata.update_authority, mint_pda, "PDA should be update authority");
    assert!(metadata.is_mutable, "Token should be mutable");
    
    // Now let's try some things that should fail
    
    // Try making another token with the same PDA (should fail)
    let result = banks
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_token(
                payer.pubkey(),
                "Dup".to_string(),
                "DUP".to_string(),
                "https://dup.json".to_string(),
            )],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        ))
        .await;
    assert!(result.is_err(), "Shouldn't be able to reuse the same PDA");

    // Try creating a token with no name (should fail)
    let result = banks
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_token(
                payer.pubkey(),
                "".to_string(),
                "TST".to_string(),
                token_uri.clone(),
            )],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        ))
        .await;
    assert!(result.is_err(), "Shouldn't allow tokens with no name");

    // Try creating a token without proper signing authority
    let non_signer = Keypair::new();
    let result = banks
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_token(
                non_signer.pubkey(),  // Using a key we don't have signing rights for
                token_name.clone(),
                token_symbol.clone(),
                token_uri.clone(),
            )],
            Some(&payer.pubkey()),
            &[&payer],  // Notice we don't have non_signer's signature
            recent_blockhash,
        ))
        .await;
    assert!(result.is_err(), "Should fail if the authority didn't actually sign");
}


*/

// THE ABOVE CODE REQUIRES SPL_TOKEN PROGRAM TO BE DEPLOYED IN THE BANKSCLIENT TEST ENVIRONMENT WHICH IS IN OUR SETUP FUNCTION

// USING RPC DEVNET
use api::prelude::*;
use solana_program::program_pack::Pack;
use solana_sdk::{
    // commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
    signer::keypair::read_keypair_file
};
use spl_token::state::Mint;
use solana_program::program_option::COption;
use mpl_token_metadata::accounts::Metadata;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::error::Error;
use steel::*;
use std::env;

// Helper function to get PDA addresses (unchanged)
fn get_addresses(_mint_seed: &[u8]) -> (Pubkey, Pubkey, u8) {
    let (mint_pda, bump) = Pubkey::find_program_address(
        &[MintAuthorityPda::SEED_PREFIX.as_bytes()],  
        &api::ID
    );
    
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint_pda.as_ref()
        ],
        &mpl_token_metadata::ID
    );
    
    (mint_pda, metadata_pda, bump)
}

// async fn request_airdrop(client: &RpcClient, pubkey: &Pubkey, amount: u64) -> Result<(), Box<dyn Error>> {
//     let signature = client.request_airdrop(pubkey, amount).await?;
//     let commitment = CommitmentConfig::confirmed();
//     client.confirm_transaction_with_commitment(&signature, commitment).await?;
//     Ok(())
// }

async fn verify_program_deployment(client: &RpcClient, program_id: Pubkey) -> Result<(), Box<dyn Error>> {
    match client.get_account(&program_id).await {
        Ok(account) => {
            println!("Program found: {:?}", program_id);
            println!("Program owner: {:?}", account.owner);
            println!("Program executable: {}", account.executable);
            Ok(())
        }
        Err(err) => Err(format!("Program not found: {}", err).into())
    }
}

async fn debug_transaction(client: &RpcClient, tx: &Transaction) {
    println!("\n=== Transaction Debug Info ===");
    println!("Number of signatures: {}", tx.signatures.len());
    println!("Signing pubkeys: {:?}", tx.message.header.num_required_signatures);
    println!("Readonly signers: {:?}", tx.message.header.num_readonly_signed_accounts);
    println!("Readonly non-signers: {:?}", tx.message.header.num_readonly_unsigned_accounts);
    
    for (i, acc) in tx.message.account_keys.iter().enumerate() {
        if let Ok(account) = client.get_account(acc).await {
            println!("\nAccount #{}: {:?}", i, acc);
            println!("  Owner: {:?}", account.owner);
            println!("  Lamports: {}", account.lamports);
            println!("  Executable: {}", account.executable);
            println!("  Data len: {}", account.data.len());
            println!("  Is signer: {}", tx.message.is_signer(i));
            println!("  Is writable: {}", tx.message.is_writable(i));
        } else {
            println!("\nAccount #{}: {:?} (not found - will be created)", i, acc);
        }
    }
}

// Helper function to load keypair from file
fn load_keypair() -> Result<Keypair, Box<dyn Error>> {
    // Try to get keypair path from environment variable
    let keypair_path = env::var("KEYPAIR_PATH")
        .unwrap_or_else(|_| format!("{}/.config/solana/id.json", env::var("HOME").unwrap()));
    
    println!("Loading keypair from: {}", keypair_path);
    Ok(read_keypair_file(&keypair_path)
        .map_err(|e| format!("Failed to load keypair: {}", e))?)
}

#[tokio::test]
async fn test_create_token() -> Result<(), Box<dyn Error>> {
    // Set up RpcClient with devnet URL using the nonblocking client
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    // Load wallet keypair
    let payer = load_keypair()?;
    println!("Using wallet address: {}", payer.pubkey());
    
    // Check wallet balance
    let balance = client.get_balance(&payer.pubkey()).await?;
    println!("Wallet balance: {} SOL", balance as f64 / 1_000_000_000.0);
    
    if balance < 1_000_000_000 {  // 1 SOL
        println!("Warning: Wallet balance is low. You may need more SOL for transactions.");
    }
    
    // Verify program deployments
    println!("Verifying program deployments...");
    println!("Verifying mpl_token_metadata::ID");
    verify_program_deployment(&client, mpl_token_metadata::ID).await?;
    println!("Verifying spl_token::ID");
    verify_program_deployment(&client, spl_token::id()).await?;
    println!("Verifying api::ID");
    verify_program_deployment(&client, api::ID).await?;
    println!("Verified all deployments");

    // Set up token info
    let token_name = "Test".to_string();
    let token_symbol = "TST".to_string();
    let token_uri = "https://test.json".to_string();
    
    let (mint_pda, metadata_pda, bump) = get_addresses(MintAuthorityPda::SEED_PREFIX.as_bytes());

    println!("\n=== PDA Info ===");
    println!("Mint PDA: {}", mint_pda);
    println!("Metadata PDA: {}", metadata_pda);
    println!("Bump: {}", bump);

    // Create token instruction
    let create_token_ix = create_token(
        payer.pubkey(),
        token_name.clone(),
        token_symbol.clone(),
        token_uri.clone()
    );

    // Set compute budget
    let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(400_000);
    let set_price_ix = ComputeBudgetInstruction::set_compute_unit_price(1);

    let recent_blockhash = client.get_latest_blockhash().await?;
    
    let tx = Transaction::new_signed_with_payer(
        &[
            compute_budget_ix,
            set_price_ix,
            create_token_ix
        ],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // Debug transaction before sending
    debug_transaction(&client, &tx).await;

    // Send and confirm transaction
    let signature = client.send_and_confirm_transaction(&tx).await?;
    println!("Transaction signature: {}", signature);

    // Verify mint account
    let mint_account = client.get_account(&mint_pda).await?;
    assert_eq!(mint_account.owner, spl_token::ID, "Mint account should be owned by Token program");
    
    let mint_data = Mint::unpack(&mint_account.data)?;
    assert_eq!(mint_data.decimals, 9, "We should have 9 decimals");
    assert!(mint_data.is_initialized, "Mint should be initialized");
    assert_eq!(
        mint_data.mint_authority,
        COption::Some(mint_pda),
        "Our PDA should be the mint authority"
    );

    // Test minting tokens
    let mint_amount = 1_000_000;
    
    let token_account = spl_associated_token_account::get_associated_token_address(
        &payer.pubkey(),
        &mint_pda
    );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint_pda,
        &spl_token::id(),
    );

    let mint_tokens_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_pda,
        &token_account,
        &mint_pda,
        &[],
        mint_amount,
    )?;

    let recent_blockhash = client.get_latest_blockhash().await?;
    
    let mint_transaction = Transaction::new_signed_with_payer(
        &[create_ata_ix, mint_tokens_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let mint_signature = client.send_and_confirm_transaction(&mint_transaction).await?;
    println!("Mint transaction signature: {}", mint_signature);

    // Verify token account
    let token_account_info = client.get_account(&token_account).await?;
    let token_account_data = spl_token::state::Account::unpack(&token_account_info.data)?;
    assert_eq!(token_account_data.amount, mint_amount, "We should have exactly the amount we minted");

    // Verify metadata
    let metadata_account = client.get_account(&metadata_pda).await?;
    assert_eq!(
        metadata_account.owner,
        mpl_token_metadata::ID,
        "Metadata account should be owned by Token Metadata program"
    );

    let metadata = Metadata::safe_deserialize(&metadata_account.data)?;
    assert_eq!(metadata.name.trim_matches(char::from(0)), token_name, "Name should match");
    assert_eq!(metadata.symbol.trim_matches(char::from(0)), token_symbol, "Symbol should match");
    assert_eq!(metadata.uri.trim_matches(char::from(0)), token_uri, "URI should match");
    assert_eq!(metadata.mint, mint_pda, "Metadata should point to our mint");
    assert_eq!(metadata.update_authority, mint_pda, "PDA should be update authority");
    assert!(metadata.is_mutable, "Token should be mutable");

    Ok(())
}

