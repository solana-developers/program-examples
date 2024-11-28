use realloc_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "realloc_program",
        realloc_api::ID,
        processor!(realloc_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

async fn check_account(
    banks_client: &mut BanksClient,
    pubkey: &Pubkey,
    expected_message: &str
) {
    let account = banks_client
        .get_account(*pubkey)
        .await
        .expect("get_account")
        .expect("account not found");

    let message_account = Message::try_from_bytes(&account.data).unwrap();
    
    // Convert stored message to string
    let stored_message = String::from_utf8(
        message_account.message[..message_account.len as usize]
            .to_vec()
    ).unwrap();

    assert_eq!(stored_message, expected_message);
    println!("Account Data Length: {}", account.data.len());
    println!("Message: {}", stored_message);
}

#[tokio::test]
async fn test_initialize() {
    let (mut banks_client, payer, recent_blockhash) = setup().await;
    let message_account = Keypair::new();
    
    let input = "hello";
    
    let ix = initialize(
        payer.pubkey(),
        message_account.pubkey(),
        input.to_string(),
    );

    let mut transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &message_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();
    
    check_account(&mut banks_client, &message_account.pubkey(), input).await;
}

#[tokio::test]
async fn test_update() {
    let (mut banks_client, payer, recent_blockhash) = setup().await;
    let message_account = Keypair::new();
    
    // First initialize
    let init_message = "hello";
    let ix = initialize(
        payer.pubkey(),
        message_account.pubkey(),
        init_message.to_string(),
    );

    let mut transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &message_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();
    
    // Then update
    let update_message = "hello world";
    let ix = update(
        payer.pubkey(),
        message_account.pubkey(),
        update_message.to_string(),
    );

    let mut transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();
    
    check_account(&mut banks_client, &message_account.pubkey(), update_message).await;
    
    // Update with shorter message
    let short_message = "hi";
    let ix = update(
        payer.pubkey(),
        message_account.pubkey(),
        short_message.to_string(),
    );

    let mut transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();
    
    check_account(&mut banks_client, &message_account.pubkey(), short_message).await;
}