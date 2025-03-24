use steel_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "steel_program",
        steel_api::ID,
        processor!(steel_program::process_instruction),
    );

    program_test.add_program("token_metadata", mpl_token_metadata::ID, None);

    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    
    //SPL TOKEN
    let token_mint_keypair = Keypair::new();

    let name = string_to_bytes::<32>("Solana Gold").unwrap();
    let symbol = string_to_bytes::<8>("GOLDSOL").unwrap();
    let uri = string_to_bytes::<128>("https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json").unwrap();
    let decimals = 9;

    let data = Token{
        name, 
        symbol,
        uri, 
        decimals
    };

    // Submit create transaction for spl_token.
    let ix = create_token(
        payer.pubkey(), 
        token_mint_keypair.pubkey(),
        data
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix], 
        Some(&payer.pubkey()), 
        &[&payer, &token_mint_keypair], 
        blockhash
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());


    //NFT
    let nft_mint_keypair = Keypair::new();
    
    let name = string_to_bytes::<32>("Homer NFT").unwrap();
    let symbol = string_to_bytes::<8>("HOMR").unwrap();
    let uri = string_to_bytes::<128>("https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json").unwrap();
    let decimals = 0;
    
    let data = Token{
        name, 
        symbol,
        uri, 
        decimals
    };
    
    //Submit create transaction for nft
    let ix = create_token(
        payer.pubkey(),
        nft_mint_keypair.pubkey(),
        data
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix], 
        Some(&payer.pubkey()), 
        &[&payer, &nft_mint_keypair], 
        blockhash
    );

    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

}


