use {
    anchor_lang::{
        solana_program::{
            instruction::Instruction,
            pubkey::Pubkey,
            system_program,
        },
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_kite::{
        create_wallet, send_transaction_from_instructions,
        token_extensions::{
            create_token_extensions_account, get_token_extensions_account_address,
            mint_tokens_to_token_extensions_account, transfer_checked_token_extensions,
            TOKEN_EXTENSIONS_PROGRAM_ID,
        },
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = non_transferable::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/non_transferable.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_create_non_transferable_mint_and_attempt_transfer() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();

    // Step 1: Create mint with NonTransferable extension via our program
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &non_transferable::instruction::Initialize {}.data(),
        non_transferable::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Verify mint account was created and has extension data
    let mint_data = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should exist");
    assert!(
        mint_data.data.len() > 82,
        "Mint should have extension data (size > 82, got {})",
        mint_data.data.len()
    );

    // Step 2: Create ATAs for sender and recipient
    let recipient = Keypair::new();
    let source_ata = create_token_extensions_account(
        &mut svm,
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    let dest_ata = create_token_extensions_account(
        &mut svm,
        &recipient.pubkey(),
        &mint_keypair.pubkey(),
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    // Step 3: Mint 1 token to sender
    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint_keypair.pubkey(),
        &source_ata,
        1,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    // Step 4: Attempt transfer — should fail because mint is NonTransferable
    let result = transfer_checked_token_extensions(
        &mut svm,
        &source_ata,
        &mint_keypair.pubkey(),
        &dest_ata,
        &payer,
        1,
        2, // decimals
        &[],
    );
    assert!(
        result.is_err(),
        "Transfer should fail because the mint is non-transferable"
    );
}
