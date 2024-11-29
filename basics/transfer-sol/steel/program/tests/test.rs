use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer,
    system_instruction::create_account, transaction::Transaction,
};
use transfer_sol_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "transfer_sol_program",
        transfer_sol_api::ID,
        processor!(transfer_sol_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    let transfer_amount = LAMPORTS_PER_SOL * 1;
    let receiver = Keypair::new();
    let receiver_1 = Keypair::new();
    let receiver_2 = Keypair::new();

    // Submit add transaction.
    let ix = transfer_sol_with_cpi(payer.pubkey(), receiver.pubkey(), transfer_amount);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //  create two account for following test
    let create_receiver_1_ix = create_account(
        &payer.pubkey(),
        &receiver_1.pubkey(),
        2 * LAMPORTS_PER_SOL,
        0,
        &transfer_sol_api::ID,
    );

    let create_receiver_2_ix = create_account(
        &payer.pubkey(),
        &receiver_2.pubkey(),
        2 * LAMPORTS_PER_SOL,
        0,
        &transfer_sol_api::ID,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_receiver_1_ix, create_receiver_2_ix],
        Some(&payer.pubkey()),
        &[&payer, &receiver_1, &receiver_2],
        blockhash,
    );

    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Submit add transaction
    let ix = transfer_sol_with_program(receiver_1.pubkey(), receiver_2.pubkey(), transfer_amount);
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &receiver_1],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
