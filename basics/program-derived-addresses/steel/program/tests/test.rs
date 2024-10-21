use program_derived_addresses_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "program_derived_addresses_program",
        program_derived_addresses_api::ID,
        processor!(program_derived_addresses_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    let test_user = Keypair::new();
    let page_visits_pda = page_visits_pda(&test_user.pubkey());

    let page_visits = PageVisits {
        page_visits: 0_u32.to_le_bytes(),
        bump: [page_visits_pda.1],
    };

    // Submit create transaction
    let ix_create = create(payer.pubkey(), test_user.pubkey(), page_visits);
    let tx_create = Transaction::new_signed_with_payer(
        &[ix_create],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res_create = banks.process_transaction(tx_create).await;
    assert!(res_create.is_ok());

    // Submit increment transaction
    let ix_increment = increment(page_visits_pda.0);
    let tx_increment = Transaction::new_signed_with_payer(
        &[ix_increment],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res_increment = banks.process_transaction(tx_increment).await;
    assert!(res_increment.is_ok());

    // Verify page visits was incremented
    let page_visits_account = banks.get_account(page_visits_pda.0).await.unwrap().unwrap();
    let page_visits = PageVisits::try_from_bytes(&page_visits_account.data).unwrap();
    assert_eq!(page_visits.page_visits(), 1);
}
