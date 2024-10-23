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

    let account_to_create = Pubkey::new_unique();
    let account_to_change = Pubkey::new_unique();

    // AccountInfo::new(
    //     &account_to_change,
    //     payer.pubkey(),
    //     true,
    //     0,
    //     data,
    //     owner,
    //     true,
    //     rent_epoch,
    // );
    // getminu

    // create_account(account_to_change., owner, seeds, system_program, payer);

    // account!()
    // ToAccount

    // // Initialize proof.
    //   create_account::<Proof>(
    //       proof_info,
    //       &ore_api::ID,
    //       &[PROOF, signer_info.key.as_ref(), &[args.bump]],
    //       system_program,
    //       payer_info,
    //   )?;

    // Submit initialize transaction.
    let ix = check_accounts(payer.pubkey(), account_to_create, account_to_change);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify counter was initialized.
    // let counter_address = counter_pda().0;
    // let counter_account = banks.get_account(counter_address).await.unwrap().unwrap();
    // let counter = Counter::try_from_bytes(&counter_account.data).unwrap();
    // assert_eq!(counter_account.owner, steel_api::ID);
    // assert_eq!(counter.value, 0);

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
