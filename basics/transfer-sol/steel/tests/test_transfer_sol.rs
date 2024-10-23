use solana_program_test::*;
use solana_sdk::{
    account::Account as SolanaAccount,
    signature::Signer,
    transaction::Transaction,
    system_instruction,
    pubkey::Pubkey,
};
use transfer_sol_program::process_instruction;

#[tokio::test]
async fn test_transfer_sol() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "transfer_sol_program",
        program_id,
        processor!(process_instruction),
    )
    .start()
    .await;

    // Create sender and receiver accounts
    let sender = payer.pubkey();
    let receiver = Pubkey::new_unique();
    let receiver_account = SolanaAccount::new(1_000_000, 0, &program_id);

    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[system_instruction::create_account(
                &payer.pubkey(),
                &receiver,
                1_000_000_000,
                0,
                &program_id,
            )],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        ))
        .await
        .unwrap();

    // Call the transfer function
    let transfer_ix = system_instruction::transfer(&sender, &receiver, 1_000_000_000);
    let tx = Transaction::new_signed_with_payer(&[transfer_ix], Some(&payer.pubkey()), &[&payer], recent_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    // Verify receiver's balance
    let receiver_account = banks_client
        .get_account(receiver)
        .await
        .expect("failed to get account")
        .expect("account not found");

    assert_eq!(receiver_account.lamports, 2_000_000_000);
}
