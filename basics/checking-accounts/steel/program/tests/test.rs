use checking_accounts_api::prelude::*;
use solana_program::hash::Hash;
use solana_program::system_program;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "checking_accounts",
        checking_accounts_api::ID,
        processor!(checking_accounts_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

async fn create_program_owned_account(
    banks: &mut BanksClient,
    payer: &Keypair,
    account: &Keypair,
    blockhash: Hash,
) -> Result<(), BanksClientError> {
    // Create a simple instruction to create and initialize an account owned by our program
    let space = 8; // Minimum space for account
    let rent = banks.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(space);

    let create_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &account.pubkey(),
        lamports,
        space as u64,
        &checking_accounts_api::ID,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[payer, account],
        blockhash,
    );

    banks.process_transaction(tx).await
}

#[tokio::test]
async fn test_successful_account_checks() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Create accounts for testing
    let account_to_create = Keypair::new();
    let account_to_change = Keypair::new();

    // First create an account owned by our program for account_to_change
    create_program_owned_account(&mut banks, &payer, &account_to_change, blockhash)
        .await
        .unwrap();

    // Submit check_accounts transaction
    let ix = check_accounts(
        payer.pubkey(),
        account_to_create.pubkey(),
        account_to_change.pubkey(),
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let result = banks.process_transaction(tx).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_wrong_owner() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Create accounts for testing
    let account_to_create = Keypair::new();
    let wrong_owner_account = Keypair::new();

    // Create account owned by system program instead of our program
    let create_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &wrong_owner_account.pubkey(),
        100000,
        0,
        &system_program::ID,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&payer, &wrong_owner_account],
        blockhash,
    );
    banks.process_transaction(tx).await.unwrap();

    // Try check_accounts with wrong owner
    let ix = check_accounts(
        payer.pubkey(),
        account_to_create.pubkey(),
        wrong_owner_account.pubkey(),
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
async fn test_fail_missing_signer() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Create accounts for testing
    let account_to_create = Keypair::new();
    let account_to_change = Keypair::new();
    let fake_payer = Keypair::new(); // This account won't actually sign

    // Create program-owned account
    create_program_owned_account(&mut banks, &payer, &account_to_change, blockhash)
        .await
        .unwrap();

    // Try check_accounts with non-signing payer
    let ix = check_accounts(
        fake_payer.pubkey(),
        account_to_create.pubkey(),
        account_to_change.pubkey(),
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer], // Notice fake_payer is not included in signers
        blockhash,
    );
    let result = banks.process_transaction(tx).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fail_invalid_system_program() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Create accounts for testing
    let account_to_create = Keypair::new();
    let account_to_change = Keypair::new();
    let fake_system_program = Keypair::new();

    // Create program-owned account
    create_program_owned_account(&mut banks, &payer, &account_to_change, blockhash)
        .await
        .unwrap();

    // Create custom instruction with wrong system program
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(account_to_create.pubkey(), false),
        AccountMeta::new(account_to_change.pubkey(), false),
        AccountMeta::new_readonly(fake_system_program.pubkey(), false),
    ];

    let ix = Instruction {
        program_id: checking_accounts_api::ID,
        accounts,
        data: CheckAccountsArgs {}.to_bytes(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let result = banks.process_transaction(tx).await;
    assert!(result.is_err());
}