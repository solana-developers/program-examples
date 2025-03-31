use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    program_pack::Pack, signature::Keypair, signer::Signer, transaction::Transaction,
};
use steel::*;
use steel_api::prelude::*;

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

    // Submit create transaction for spl_token.
    let ix = create_token(
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

    let mint_account_data = banks
        .get_account(token_mint_keypair.pubkey())
        .await
        .unwrap()
        .unwrap()
        .data;
    let deserialized_mint_data = spl_token::state::Mint::unpack(&mint_account_data).unwrap();
    assert!(deserialized_mint_data.is_initialized);
    assert_eq!(deserialized_mint_data.decimals, decimals);
    assert_eq!(
        deserialized_mint_data.mint_authority.unwrap(),
        payer.pubkey()
    );

    let metadata_pda = Pubkey::find_program_address(
        &[
            METADATA,
            mpl_token_metadata::ID.as_ref(),
            token_mint_keypair.pubkey().as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;
    let metadata_account_data = banks.get_account(metadata_pda).await.unwrap().unwrap().data;
    let deserialized_metadata_data =
        mpl_token_metadata::accounts::Metadata::from_bytes(&metadata_account_data).unwrap();
    assert_eq!(deserialized_metadata_data.update_authority, payer.pubkey());
    assert_eq!(deserialized_metadata_data.mint, token_mint_keypair.pubkey());

    //NFT
    let nft_mint_keypair = Keypair::new();

    let name = string_to_bytes::<32>("Homer NFT").unwrap();
    let symbol = string_to_bytes::<8>("HOMR").unwrap();
    let uri = string_to_bytes::<128>("https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json").unwrap();
    let decimals = 0;

    //Submit create transaction for nft
    let ix = create_token(
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
