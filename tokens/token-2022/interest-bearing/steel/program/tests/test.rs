use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token_2022::{extension::{interest_bearing_mint::InterestBearingConfig, BaseStateWithExtensions, StateWithExtensions}, state::Mint};
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
    let mint = Keypair::new();
    
    // Submit initialize transaction.
    let ix = initialize(payer.pubkey(), mint.pubkey(), 100);
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

     let mint_data = mint_account.data.as_slice();
     let state_with_extensions = StateWithExtensions::<Mint>::unpack(mint_data).unwrap();
     let extension = state_with_extensions
         .get_extension::<InterestBearingConfig>()
         .unwrap();
 
     assert_eq!(
         i16::from_le_bytes(extension.current_rate.0),
         100,
         "Interest Rate Mismatch"
     );
 
    // Update Rate
    let ix: Instruction = update_rate(payer.pubkey(), mint.pubkey(), 1500);
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());


     //Get mint data
     let mint_account = banks.get_account(mint.pubkey()).await.unwrap().unwrap();

     let mint_data = mint_account.data.as_slice();
     let state_with_extensions = StateWithExtensions::<Mint>::unpack(mint_data).unwrap();
     let extension = state_with_extensions
         .get_extension::<InterestBearingConfig>()
         .unwrap();
 
     assert_eq!(
         i16::from_le_bytes(extension.current_rate.0),
         1500,
         "Interest Rate Mismatch"
     );

}
