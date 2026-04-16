use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

#[test]
fn test_say_hello() {
    let program_id = hello_solana::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/hello_solana.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 1_000_000_000).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &hello_solana::instruction::Hello {}.data(),
        hello_solana::accounts::Hello {}.to_account_metas(None),
    );

    send_transaction_from_instructions(&mut svm, vec![instruction], &[&payer], &payer.pubkey())
        .unwrap();
}
