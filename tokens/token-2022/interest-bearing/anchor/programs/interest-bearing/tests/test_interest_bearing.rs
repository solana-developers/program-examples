use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_kite::{
        create_wallet, send_transaction_from_instructions,
        token_extensions::TOKEN_EXTENSIONS_PROGRAM_ID,
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = interest_bearing::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/interest_bearing.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_initialize_and_update_rate() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();

    // Step 1: Initialize mint with InterestBearingConfig extension (rate=0)
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &interest_bearing::instruction::Initialize { rate: 0 }.data(),
        interest_bearing::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();

    // Verify mint account exists
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should exist");
    assert!(!mint_account.data.is_empty(), "Mint should have data");

    svm.expire_blockhash();

    // Step 2: Update the interest rate to 100
    let update_rate_ix = Instruction::new_with_bytes(
        program_id,
        &interest_bearing::instruction::UpdateRate { rate: 100 }.data(),
        interest_bearing::accounts::UpdateRate {
            authority: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![update_rate_ix], &[&payer], &payer.pubkey()).unwrap();

    // Verify mint still exists after rate update
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should still exist");
    assert!(!mint_account.data.is_empty(), "Mint should still have data");
}
