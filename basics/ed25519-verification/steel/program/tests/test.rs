use {
    ed25519_custodial_api::ed25519_custodial,
    solana_program::{pubkey::Pubkey, system_program},
    solana_program_test::*,
    solana_sdk::{signature::Keypair, signer::Signer},
    steel_test::*,
};

#[tokio::test]
async fn test_ed25519_transfer() {
    let program_id = Pubkey::new_unique();
    let mut context = program_test()
        .add_program("ed25519_custodial", program_id)
        .start_with_context()
        .await;

    let custodial_account = Keypair::new();
    let recipient = Keypair::new();
    let signer = Keypair::new();
    let amount = 1_000_000;

    // Create test message and signature
    let message = format!("Transfer {} lamports to {}", amount, recipient.pubkey());
    let signature = signer.sign_message(message.as_bytes());

    let accounts = ed25519_custodial::TransferAccounts {
        custodial_account: custodial_account.pubkey(),
        recipient: recipient.pubkey(),
        signer: signer.pubkey(),
    };

    let ix = ed25519_custodial::transfer(
        program_id,
        accounts,
        signature.to_bytes(),
        signer.pubkey().to_bytes(),
        message.as_bytes().to_vec(),
        amount,
    );

    let result = context
        .banks_client
        .process_transaction(&[ix], &[&signer])
        .await;

    assert!(result.is_ok());
} 