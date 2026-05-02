use {
    anchor_lang::{
        solana_program::{instruction::Instruction, system_instruction, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

#[test]
fn test_check_accounts() {
    let program_id = checking_account_program::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/checking_account_program.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let account_to_change = Keypair::new();
    let account_to_create = Keypair::new();

    // First, create an account owned by our program (like the TS test does)
    let rent_exempt_balance = svm.minimum_balance_for_rent_exemption(0);
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &account_to_change.pubkey(),
        rent_exempt_balance,
        0,
        &program_id,
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![create_account_ix],
        &[&payer, &account_to_change],
        &payer.pubkey(),
    )
    .unwrap();

    svm.expire_blockhash();

    // Now call check_accounts
    let check_accounts_ix = Instruction::new_with_bytes(
        program_id,
        &checking_account_program::instruction::CheckAccounts {}.data(),
        checking_account_program::accounts::CheckingAccounts {
            payer: payer.pubkey(),
            account_to_create: account_to_create.pubkey(),
            account_to_change: account_to_change.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![check_accounts_ix],
        &[&payer],
        &payer.pubkey(),
    )
    .unwrap();
}
