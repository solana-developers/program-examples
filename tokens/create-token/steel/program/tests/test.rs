use create_token_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "create_token_program",
        create_token_api::ID,
        processor!(create_token_program::process_instruction),
    );

    program_test.add_program("token_metadata", mpl_token_metadata::ID, None);

    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let token_mint_keypair = Keypair::new();

    let name = str_to_bytes::<32>("Solana Gold");
    let symbol = str_to_bytes::<8>("GOLDSOL");
    let uri = str_to_bytes::<64>("https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json");
    let decimals = 9;

    // Submit create transaction for spl token.
    let ix = create(
        payer.pubkey(),
        token_mint_keypair.pubkey(),
        name,
        symbol,
        uri,
        decimals,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &token_mint_keypair],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let nft_mint_keypair = Keypair::new();

    let name = str_to_bytes::<32>("Homer NFT");
    let symbol = str_to_bytes::<8>("HOMR");
    let uri = str_to_bytes::<64>("https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json");
    let decimals = 0;

    // Submit create transaction for NFT.
    let ix = create(
        payer.pubkey(),
        nft_mint_keypair.pubkey(),
        name,
        symbol,
        uri,
        decimals,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &nft_mint_keypair],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());
}
