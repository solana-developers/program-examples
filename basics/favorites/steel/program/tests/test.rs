use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
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

    let favorite_number: u64 = 23;
    let favorite_color: &str = "purple";
    let mut favorite_hobbies: Vec<&str> = Vec::new();
    favorite_hobbies.push("skiing");
    favorite_hobbies.push("skydiving");
    favorite_hobbies.push("biking");

    // Submit set favorites transaction.
    let ix = set_favorites(
        payer.pubkey(),
        favorite_number,
        favorite_color,
        favorite_hobbies,
    );
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify favorites was updated.
    let favorites_address = favorites_pda().0;
    let favorites_account = banks.get_account(favorites_address).await.unwrap().unwrap();
    let favorites = Favorites::try_from_bytes(&favorites_account.data).unwrap();

    let favorites_number = favorites.number;
    let favorites_color = bytes32_to_string(&favorites.color).unwrap();
    let favorites_hobbies = bytes32_array_to_strings(&favorites.hobbies).unwrap();

    assert_eq!(favorites_account.owner, steel_api::ID);
    assert_eq!(favorites_number, 23);
    assert_eq!(favorites_color, "purple");
    assert_eq!(favorites_hobbies[0], "skiing");
}
