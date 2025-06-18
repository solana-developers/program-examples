use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token_2022::{
    extension::{
        transfer_fee::TransferFeeConfig, BaseStateWithExtensions, ExtensionType,
        StateWithExtensions,
    },
    state::Mint,
};
use steel_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "steel_program",
        steel_api::ID,
        processor!(steel_program::process_instruction),
    );
    //Custom SPL Token 2022
    // program_test.add_program("spl_token_2022", spl_token_2022::ID, None);
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let to = Keypair::new();
    let mint = Keypair::new();

    // Create account for mint
    // let create_account_ix = system_instruction::create_account(
    //     &payer.pubkey(),
    //     &to.pubkey(),
    //     (Rent::get().unwrap()).minimum_balance(Mint::LEN),
    //     Mint::LEN as u64,
    //     &spl_token_2022::ID,
    // );

    let maximum_fee = 1000;
    let transfer_fee_basis_points = 1000;

    // Submit initialize transaction.
    let ix = initialize(
        payer.pubkey(),
        mint.pubkey(),
        maximum_fee,
        transfer_fee_basis_points,
        6,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let mint_account = banks
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .expect("Mint account not found");

    let mint_data = mint_account.data.as_slice();
    // let mint_state = Mint::unpack(mint_data).expect("Failed to unpack mint");

    let state_with_extensions = StateWithExtensions::<Mint>::unpack(mint_data).unwrap();
    let extension_types: Vec<_> = state_with_extensions.get_extension_types().unwrap();
    // extension_types.iter().for_each(|ext| println!("NOthing? {:?}", ext));

    let extension = state_with_extensions
        .get_extension::<TransferFeeConfig>()
        .unwrap();
    dbg!(extension.transfer_fee_config_authority.0);
    dbg!(payer.pubkey());
    assert_eq!(
        extension.transfer_fee_config_authority.0,
        payer.pubkey(),
        "Transfer fee config authority mismatch"
    );

    assert!(
        extension_types.contains(&ExtensionType::TransferFeeConfig),
        "TransferFeeConfig extension not found in mint account"
    );
}
