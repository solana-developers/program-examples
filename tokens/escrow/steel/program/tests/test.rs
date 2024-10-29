use api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_program::system_instruction;
use solana_sdk::{
    signature::{Keypair, read_keypair_file}, 
    signer::Signer,
    transaction::Transaction,
};
use spl_token::instruction as token_instruction;
use std::{env, error::Error};

// Helper function to load keypair from config
fn load_keypair() -> Result<Keypair, Box<dyn Error>> {
    let keypair_path = env::var("KEYPAIR_PATH")
        .unwrap_or_else(|_| format!("{}/.config/solana/id.json", env::var("HOME").unwrap()));
    println!("Loading keypair from: {}", keypair_path);
    Ok(read_keypair_file(&keypair_path)
        .map_err(|e| format!("Failed to load keypair: {}", e))?)
}

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "swap_program",
        api::ID,
        processor!(process_instruction),
    );

    let payer = load_keypair().unwrap_or_else(|_| {
        println!("Failed to load keypair, using new random keypair");
        Keypair::new()
    });
    
    program_test.prefer_bpf(true);
    let (banks, _payer, blockhash) = program_test.start().await;
    
    (banks, payer, blockhash)
}

// Helper to create a new mint
async fn create_test_mint(
    banks: &mut BanksClient,
    payer: &Keypair,
    mint_authority: &Keypair,
    blockhash: Hash,
) -> Pubkey {
    let mint = Keypair::new();
    
    let rent = banks.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

    let create_acc_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    let init_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &mint_authority.pubkey(),
        None,
        0,
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_acc_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[payer, &mint, mint_authority],
        blockhash,
    );

    banks.process_transaction(tx).await.unwrap();
    mint.pubkey()
}

// Helper to create a token account
async fn create_test_token_account(
    banks: &mut BanksClient,
    payer: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
    blockhash: Hash,
) -> Pubkey {
    let ata = spl_associated_token_account::get_associated_token_address(
        owner,
        mint,
    );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        owner,
        mint,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    banks.process_transaction(tx).await.unwrap();
    ata
}

// Helper to mint tokens
async fn mint_test_tokens(
    banks: &mut BanksClient,
    payer: &Keypair,
    mint: &Pubkey,
    dest: &Pubkey,
    mint_authority: &Keypair,
    amount: u64,
    blockhash: Hash,
) {
    let mint_ix = token_instruction::mint_to(
        &spl_token::id(),
        mint,
        dest,
        &mint_authority.pubkey(),
        &[],
        amount,
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&payer.pubkey()),
        &[payer, mint_authority],
        blockhash,
    );

    banks.process_transaction(tx).await.unwrap();
}

// Helper to get token balance
async fn get_token_balance(banks: &mut BanksClient, account: Pubkey) -> u64 {
    banks.get_account(account).await
        .unwrap()
        .unwrap()
        .data
        .as_slice()
        .try_into_token_account()
        .unwrap()
        .amount
}

#[tokio::test]
async fn test_full_swap_workflow() {
    // Setup test environment
    let (mut banks, payer, blockhash) = setup().await;

    // Setup tokens
    let mint_authority = Keypair::new();
    let mint_a = create_test_mint(&mut banks, &payer, &mint_authority, blockhash).await;
    let mint_b = create_test_mint(&mut banks, &payer, &mint_authority, blockhash).await;

    // Setup maker accounts
    let maker_token_a = create_test_token_account(
        &mut banks,
        &payer,
        &mint_a,
        &payer.pubkey(),
        blockhash,
    ).await;

    let maker_token_b = create_test_token_account(
        &mut banks,
        &payer,
        &mint_b,
        &payer.pubkey(),
        blockhash,
    ).await;

    // Setup taker
    let taker = Keypair::new();
    let taker_token_a = create_test_token_account(
        &mut banks,
        &payer,
        &mint_a,
        &taker.pubkey(),
        blockhash,
    ).await;

    let taker_token_b = create_test_token_account(
        &mut banks,
        &payer,
        &mint_b,
        &taker.pubkey(),
        blockhash,
    ).await;

    // Mint initial tokens
    mint_test_tokens(
        &mut banks,
        &payer,
        &mint_a,
        &maker_token_a,
        &mint_authority,
        1000,
        blockhash,
    ).await;

    mint_test_tokens(
        &mut banks,
        &payer,
        &mint_b,
        &taker_token_b,
        &mint_authority,
        1000,
        blockhash,
    ).await;

    // Test make offer
    let offer_id = 1;
    let offered_amount = 100;
    let wanted_amount = 200;

    let make_offer_ix = make_offer(
        payer.pubkey(),
        mint_a,
        mint_b,
        maker_token_a,
        offer_id,
        offered_amount,
        wanted_amount,
    );

    let tx = Transaction::new_signed_with_payer(
        &[make_offer_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    banks.process_transaction(tx).await.unwrap();

    // Verify offer creation
    let (offer_key, _) = get_offer_address(&payer.pubkey(), offer_id);
    let offer_account = banks.get_account(offer_key).await.unwrap().unwrap();
    let offer_data = Offer::try_from_bytes(&offer_account.data).unwrap();
    
    assert_eq!(offer_data.maker, payer.pubkey());
    assert_eq!(offer_data.token_mint_a, mint_a);
    assert_eq!(offer_data.token_mint_b, mint_b);
    assert_eq!(offer_data.token_b_wanted_amount, wanted_amount);

    // Test take offer
    let take_offer_ix = take_offer(
        taker.pubkey(),
        payer.pubkey(),
        mint_a,
        mint_b,
        taker_token_a,
        taker_token_b,
        maker_token_b,
        offer_id,
    );

    let tx = Transaction::new_signed_with_payer(
        &[take_offer_ix],
        Some(&payer.pubkey()),
        &[&payer, &taker],
        blockhash,
    );

    banks.process_transaction(tx).await.unwrap();

    // Verify final balances
    let maker_b_balance = get_token_balance(&mut banks, maker_token_b).await;
    let taker_a_balance = get_token_balance(&mut banks, taker_token_a).await;

    assert_eq!(maker_b_balance, wanted_amount);
    assert_eq!(taker_a_balance, offered_amount);

    // Verify offer account was closed
    let closed_offer = banks.get_account(offer_key).await.unwrap();
    assert!(closed_offer.is_none());
}

#[tokio::test]
async fn test_invalid_offer_creation() {
    let (mut banks, payer, blockhash) = setup().await;

    // Try to create offer with invalid amounts
    let mint_a = Pubkey::new_unique();
    let mint_b = Pubkey::new_unique();
    let token_account = Pubkey::new_unique();

    let ix = make_offer(
        payer.pubkey(),
        mint_a,
        mint_b,
        token_account,
        1,
        0,  // Invalid zero amount
        100,
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let result = banks.process_transaction(tx).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_unauthorized_take_offer() {
    let (mut banks, payer, blockhash) = setup().await;

    // Setup tokens and accounts like in full workflow test
    let mint_authority = Keypair::new();
    let mint_a = create_test_mint(&mut banks, &payer, &mint_authority, blockhash).await;
    let mint_b = create_test_mint(&mut banks, &payer, &mint_authority, blockhash).await;
    
    // Create offer
    let maker_token_a = create_test_token_account(
        &mut banks,
        &payer,
        &mint_a,
        &payer.pubkey(),
        blockhash,
    ).await;

    mint_test_tokens(
        &mut banks,
        &payer,
        &mint_a,
        &maker_token_a,
        &mint_authority,
        1000,
        blockhash,
    ).await;

    let offer_id = 1;
    let make_offer_ix = make_offer(
        payer.pubkey(),
        mint_a,
        mint_b,
        maker_token_a,
        offer_id,
        100,
        200,
    );

    let tx = Transaction::new_signed_with_payer(
        &[make_offer_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    banks.process_transaction(tx).await.unwrap();

    // Try to take offer without proper signing
    let unauthorized = Keypair::new();
    let taker_token_a = create_test_token_account(
        &mut banks,
        &payer,
        &mint_a,
        &unauthorized.pubkey(),
        blockhash,
    ).await;

    let taker_token_b = create_test_token_account(
        &mut banks,
        &payer,
        &mint_b,
        &unauthorized.pubkey(),
        blockhash,
    ).await;

    let take_offer_ix = take_offer(
        unauthorized.pubkey(),
        payer.pubkey(),
        mint_a,
        mint_b,
        taker_token_a,
        taker_token_b,
        maker_token_a,
        offer_id,
    );

    let tx = Transaction::new_signed_with_payer(
        &[take_offer_ix],
        Some(&payer.pubkey()),
        &[&payer], // Missing unauthorized signer
        blockhash,
    );

    let result = banks.process_transaction(tx).await;
    assert!(result.is_err());
}