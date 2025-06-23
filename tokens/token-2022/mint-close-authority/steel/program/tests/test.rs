use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token_2022::{
    extension::{
        mint_close_authority::MintCloseAuthority,
        transfer_fee::{TransferFeeAmount, TransferFeeConfig},
        BaseStateWithExtensions, ExtensionType, StateWithExtensions,
    },
    instruction::mint_to,
    state::{Account, Mint},
};
use steel::*;
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
    let recv = Keypair::new();
    let mint = Keypair::new();

    // Submit initialize transaction.
    let ix = initialize(payer.pubkey(), mint.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //Get mint data
    let mint_account = banks.get_account(mint.pubkey()).await.unwrap().unwrap();
    dbg!(mint_account.lamports);
    let mint_data = mint_account.data.as_slice();
    let state_with_extensions = StateWithExtensions::<Mint>::unpack(mint_data).unwrap();
    let extension_types = state_with_extensions.get_extension_types().unwrap();
    let extension = state_with_extensions
        .get_extension::<MintCloseAuthority>()
        .unwrap();

    assert_eq!(
        extension.close_authority.0,
        payer.pubkey(),
        "Mint close authority mismatch"
    );

    assert!(
        extension_types.contains(&ExtensionType::MintCloseAuthority),
        "Mint Close Authority extension not found in mint account"
    );

    // Submit close mint transaction.
    let ix = mint_close(payer.pubkey(), recv.pubkey(), mint.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    //Get mint data
    let mint_account = banks.get_account(mint.pubkey()).await.unwrap();
    assert!(mint_account.is_none(), "Mint account should be closed but still exists");

    //Checking if the rent was transferred to the destination account
    let recv_account = banks.get_account(recv.pubkey()).await.unwrap().unwrap();
    assert!(recv_account.lamports == 2296800);
}
