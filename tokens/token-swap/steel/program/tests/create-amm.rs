mod helpers;
use helpers::*;

#[tokio::test]
async fn creates_amm_successfully() {
    let (mut banks, payer, blockhash) = setup().await;

    let values = TestValues::new(&token_swap_api::ID);

    let res = creates_amm(&mut banks, &payer, blockhash, &values).await;

    assert!(res.is_ok());
}
#[tokio::test]
async fn create_amm_failed_due_to_invalid_fee_too_high() {
    let (mut banks, payer, blockhash) = setup().await;

    let mut values = TestValues::new(&token_swap_api::ID);
    values.fee = 20000;

    let res = creates_amm(&mut banks, &payer, blockhash, &values).await;

    assert!(res.is_err())
}
