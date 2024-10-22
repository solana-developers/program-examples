use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;
use steel_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "steel",
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
    let favorite_color: String = String::from("purple");
    let mut favorite_hobbies: Vec<String> = Vec::new();
    favorite_hobbies.push(String::from("skiing"));
    favorite_hobbies.push(String::from("skydiving"));
    favorite_hobbies.push(String::from("biking"));

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

    // Verify counter was initialized.
    let favorites_address = favorites_pda().0;
    let favorites_account = banks.get_account(favorites_address).await.unwrap().unwrap();
    let favorites = Favorites::try_from_bytes(&favorites_account.data).unwrap();
    assert_eq!(favorites_account.owner, steel_api::ID);
    assert_eq!(favorites.number, 23);

    // Submit add transaction.
    // let ix = add(payer.pubkey(), 42);
    // let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    // let res = banks.process_transaction(tx).await;
    // assert!(res.is_ok());

    // // Verify counter was incremented.
    // let counter_account = banks.get_account(counter_address).await.unwrap().unwrap();
    // let counter = Counter::try_from_bytes(&counter_account.data).unwrap();
    // assert_eq!(counter.value, 42);
}
