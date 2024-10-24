use std::vec;

use solana_program::hash::Hash;

use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    config::program, lamports, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use steel::*;
use transfer_sol_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "transfer_sol_program",
        transfer_sol_api::ID,
        processor!(transfer_sol_program::process_instruction),
    );

    program_test.prefer_bpf(true);
    program_test.start().await
}

fn create_system_program_account_info() -> AccountInfo<'static> {
    let mut data = vec![];
    let mut lamports = 0;
    AccountInfo::new(
        &system_program::ID,
        false,
        false,
        &mut lamports,
        &mut data,
        &system_program::ID,
        true,
        0,
    )
}

//     let system_program_id = &system_program::ID;
//     let key = Rc::new(*system_program_id);
//     let lamports = Rc::new(RefCell::new(0));
//     let data = Rc::new(RefCell::new(vec![]));
//     let owner = system_program_id;
//     let is_signer = false;
//     let is_writable = false;

//     AccountInfo {
//         key: &*key,
//         is_signer,
//         is_writable,
//         lamports,
//         data,
//         owner,
//         executable: true,
//         rent_epoch: 0,
//     }
// }

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // 1 SOL
    let amount = 1 * LAMPORTS_PER_SOL;

    // Generate a new keypair for the recipient
    let recipient = Keypair::new();

    // Submit transfer with cpi transaction.
    let ix = with_cpi(payer.pubkey(), recipient.pubkey(), amount);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // // Verify counter was initialized.
    // let counter_address = counter_pda().0;
    // let counter_account = banks.get_account(counter_address).await.unwrap().unwrap();
    // let counter = Counter::try_from_bytes(&counter_account.data).unwrap();
    // assert_eq!(counter_account.owner, transfer_sol_api::ID);
    // assert_eq!(counter.value, 0);

    // Generate a new keypair to create an account owned by our program
    let program_owned_account = Keypair::new();
    let system_program_account = create_system_program_account_info();

    create_account(
        &program_owned_account,
        &system_program_account,
        &payer.pubkey(),
        &program::ID,
        &[&payer, &system_program_account],
    )?;

    // Submit transfer with program transaction.
    let ix = with_program(payer.pubkey(), recipient.pubkey(), amount);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // // Verify counter was incremented.
    // let counter_account = banks.get_account(counter_address).await.unwrap().unwrap();
    // let counter = Counter::try_from_bytes(&counter_account.data).unwrap();
    // assert_eq!(counter.value, 42);
}
