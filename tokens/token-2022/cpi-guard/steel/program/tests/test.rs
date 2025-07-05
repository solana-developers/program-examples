use solana_program::{hash::Hash, program_pack::Pack};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    signature::Keypair, signer::Signer, system_instruction::create_account,
    transaction::Transaction,
};
use spl_token_2022::{
    extension::{cpi_guard::instruction::enable_cpi_guard, ExtensionType},
    instruction::{initialize_account3, initialize_mint2},
    state::{Account, Mint},
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

    //Setup Mint.
    let create_mint_ix = create_account(
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

    //Setup Token Account
    let space =
        ExtensionType::try_calculate_account_len::<Account>(&[ExtensionType::CpiGuard]).unwrap();

    let create_acc_ix = create_account(
        &payer.pubkey(),
        &token_acc.pubkey(),
        banks.get_rent().await.unwrap().minimum_balance(space),
        space as u64,
        &spl_token_2022::ID,
    );

    let init_acc_ix = initialize_account3(
        &spl_token_2022::ID,
        &token_acc.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
    )
    .unwrap();

    //Initialize Cpi Guard Extension
    let enable_cpi_guard_ix = enable_cpi_guard(
        &spl_token_2022::ID,
        &token_acc.pubkey(),
        &payer.pubkey(),
        &[],
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            create_mint_ix,
            init_mint_ix,
            create_acc_ix,
            init_acc_ix,
            enable_cpi_guard_ix,
        ],
        Some(&payer.pubkey()),
        &[&payer, &mint, &token_acc],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //Try to burn tokens via cpi, should fail
    let cpi_burn_ix = cpi_burn(payer.pubkey(), mint.pubkey(), token_acc.pubkey());

    let tx = Transaction::new_signed_with_payer(
        &[cpi_burn_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;

    let err_string = format!("{:?}", res);

    assert!(
        err_string.contains("Custom(43)"),
        "Expected TokenError::CpiGuardBurnBlocked(43) , got: {}",
        err_string
    );
}
