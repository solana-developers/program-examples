use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token_minter_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "spl_token_minter_program",
        spl_token_minter_api::ID,
        processor!(spl_token_minter_program::process_instruction),
    );

    program_test.add_program("token_metadata", mpl_token_metadata::ID, None);

    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let mint_keypair = Keypair::new();

    let name = str_to_bytes::<32>("Solana Gold");
    let symbol = str_to_bytes::<8>("GOLDSOL");
    let uri = str_to_bytes::<64>("https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json");

    // Submit create transaction.
    let ix = create(payer.pubkey(), mint_keypair.pubkey(), name, symbol, uri);
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_keypair],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let recipient = Keypair::new();

    let to_ata = spl_associated_token_account::get_associated_token_address(
        &recipient.pubkey(),
        &mint_keypair.pubkey(),
    );

    // Submit mint transaction.
    let ix = mint(
        payer.pubkey(),
        recipient.pubkey(),
        mint_keypair.pubkey(),
        to_ata,
        100,
    );
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
