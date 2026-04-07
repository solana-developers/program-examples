use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

fn setup() -> (LiteSVM, Keypair) {
    let program_id = close_account_program::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/close_account_program.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, payer)
}

#[test]
fn test_create_and_close_user() {
    let (mut svm, payer) = setup();
    let program_id = close_account_program::id();

    // Derive the PDA for the user's account
    let (user_account_pda, _bump) =
        Pubkey::find_program_address(&[b"USER", payer.pubkey().as_ref()], &program_id);

    // Create user
    let create_ix = Instruction::new_with_bytes(
        program_id,
        &close_account_program::instruction::CreateUser {
            name: "John Doe".to_string(),
        }
        .data(),
        close_account_program::accounts::CreateUserContext {
            user: payer.pubkey(),
            user_account: user_account_pda,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![create_ix], &[&payer], &payer.pubkey())
        .unwrap();

    // Verify account exists and has correct data
    let account = svm
        .get_account(&user_account_pda)
        .expect("Account should exist after creation");
    assert!(account.data.len() > 0, "Account should have data");

    svm.expire_blockhash();

    // Close user
    let close_ix = Instruction::new_with_bytes(
        program_id,
        &close_account_program::instruction::CloseUser {}.data(),
        close_account_program::accounts::CloseUserContext {
            user: payer.pubkey(),
            user_account: user_account_pda,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![close_ix], &[&payer], &payer.pubkey())
        .unwrap();

    // Verify account is closed
    let account = svm.get_account(&user_account_pda);
    assert!(account.is_none(), "Account should be closed");
}
