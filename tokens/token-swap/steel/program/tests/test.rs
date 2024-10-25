use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer, system_instruction,
    transaction::Transaction,
};
use steel::*;
use token_swap_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "token_swap_program",
        token_swap_api::ID,
        processor!(token_swap_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    let admin = Keypair::new();
    let id = Keypair::new();
    let fee = 1000; // 10%

    // create admin account
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::create_account(
            &payer.pubkey(),
            &admin.pubkey(),
            1 * LAMPORTS_PER_SOL,
            0,
            &token_swap_api::ID,
        )],
        Some(&payer.pubkey()),
        &[&payer, &admin],
        blockhash,
    );

    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Submit initialize transaction.
    let ix = create_amm(payer.pubkey(), admin.pubkey(), id.pubkey(), fee);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;

    assert!(res.is_ok());

    // Verify counter was initialized.
    // let amm_address = amm_pda(id.pubkey()).0;
    // let amm_account = banks.get_account(amm_address).await.unwrap().unwrap();
    // let amm = Amm::try_from_bytes(&amm_account.data).unwrap();
    // assert_eq!(amm_account.owner, token_swap_api::ID);
    // assert_eq!(amm.id, id.pubkey());
    // assert_eq!(amm.admin, admin.pubkey());
    // assert_eq!(amm.fee, fee);

    // // Submit add transaction.
    // let ix = add(payer.pubkey(), 42);
    // let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    // let res = banks.process_transaction(tx).await;
    // assert!(res.is_ok());

    // // Verify counter was incremented.
    // let counter_account = banks.get_account(amm_address).await.unwrap().unwrap();
    // let counter = Counter::try_from_bytes(&counter_account.data).unwrap();
    // assert_eq!(counter.value, 42);
}
