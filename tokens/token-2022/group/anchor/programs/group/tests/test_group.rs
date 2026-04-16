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
    solana_signer::Signer,
};

#[test]
fn test_initialize_group() {
    let program_id = group::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/group.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    // Derive the mint PDA
    let (mint_account, _bump) = Pubkey::find_program_address(&[b"group"], &program_id);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &group::instruction::TestInitializeGroup {}.data(),
        group::accounts::InitializeGroup {
            payer: payer.pubkey(),
            mint_account,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(&mut svm, vec![instruction], &[&payer], &payer.pubkey()).unwrap();

    // Verify mint was created with group pointer extension
    let mint_data = svm
        .get_account(&mint_account)
        .expect("Mint account should exist");
    assert!(
        mint_data.data.len() > 82,
        "Mint should have extension data (size > 82, got {})",
        mint_data.data.len()
    );
}
