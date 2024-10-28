mod helpers;
use helpers::*;
use solana_sdk::{signature::Keypair, signer::Signer};
use steel::*;
use token_swap_api::prelude::*;

#[tokio::test]
async fn creates_pool_successfully() {
    let (mut banks, payer, blockhash) = setup().await;
    let values = TestValues::new(&token_swap_api::ID);

    let _ = creates_amm(&mut banks, &payer, blockhash, &values).await;

    let res = creates_pool(&mut banks, &payer, blockhash, &values).await;

    assert!(res.is_ok());

    //Verify pool contains corrects details
    let pool_account = banks
        .get_account(values.pool_key)
        .await
        .unwrap()
        .expect("could not fetch account");
    let pool_info = Pool::try_from_bytes(&pool_account.data).unwrap();
    assert_eq!(pool_info.amm, values.amm_key);
    assert_eq!(pool_info.mint_a, values.mint_a_keypair.pubkey());
    assert_eq!(pool_info.mint_b, values.mint_b_keypair.pubkey());
}

#[tokio::test]
async fn fails_to_create_pool_invalild_mint() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let mut test_values = TestValues::new(&token_swap_api::ID);
    test_values.mint_b_keypair = Keypair::new();

    let _ = creates_amm(&mut banks, &payer, blockhash, &test_values).await;

    let res = creates_pool(&mut banks, &payer, blockhash, &test_values).await;

    assert!(res.is_err());
}
