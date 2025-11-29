use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    program_pack::Pack, signature::Keypair, signer::Signer, system_instruction::create_account, transaction::Transaction
};

use spl_memo::build_memo;
use spl_token_2022::{
    instruction::{initialize_mint2, mint_to, transfer_checked},
    state::Mint,
};
use steel_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "steel_program",
        steel_api::ID,
        processor!(steel_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    let mint = Keypair::new();
    let token_acc = Keypair::new();
    let receiver_token_acc = Keypair::new();

    let create_acc_ix = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        banks.get_rent().await.unwrap().minimum_balance(Mint::LEN),
        Mint::LEN as u64,
        &spl_token_2022::ID,
    );
    let init_mint_ix = initialize_mint2(
        &spl_token_2022::ID,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        6,
    )
    .unwrap();
    let mint_to_ix = mint_to(
        &spl_token_2022::ID,
        &mint.pubkey(),
        &token_acc.pubkey(),
        &payer.pubkey(),
        &[],
        1_000_000,
    )
    .unwrap();

    // Submit initialize transaction.
    let ix = initialize(payer.pubkey(), mint.pubkey(), token_acc.pubkey());
    let ix2 = initialize(payer.pubkey(), mint.pubkey(), receiver_token_acc.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[create_acc_ix, init_mint_ix, ix, ix2, mint_to_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint, &token_acc, &receiver_token_acc],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Try transfer without memo
    let transfer_ix = transfer_checked(
        &spl_token_2022::ID,
        &token_acc.pubkey(),
        &mint.pubkey(),
        &receiver_token_acc.pubkey(),
        &payer.pubkey(),
        &[],
        100,
        6,
    )
    .unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    let err_string = format!("{:?}", res);

    // Custom(0x24) = Custom(36) = TokenError::MissingMemo
    assert!(
        err_string.contains("Custom(36)"),
        "Expected TokenError::MissingMemo (36), got: {}",
        err_string
    );

    //Try Transfer with memo
    let memo = "Memo from Steel Token2022 Examples";
    let memo_ix = build_memo(memo.as_bytes(), &[&payer.pubkey()]);
    let transfer_ix = transfer_checked(
        &spl_token_2022::ID,
        &token_acc.pubkey(),
        &mint.pubkey(),
        &receiver_token_acc.pubkey(),
        &payer.pubkey(),
        &[],
        100,
        6,
    )
    .unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[memo_ix, transfer_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
