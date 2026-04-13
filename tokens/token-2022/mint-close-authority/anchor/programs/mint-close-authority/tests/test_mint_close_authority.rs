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
    let program_id = mint_close_authority::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/mint_close_authority.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_create_and_close_mint() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();

    // Step 1: Create Mint with Close Authority
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &mint_close_authority::instruction::Initialize {}.data(),
        mint_close_authority::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();

    // Verify mint exists
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should exist");
    assert!(!mint_account.data.is_empty(), "Mint should have data");

    svm.expire_blockhash();

    // Step 2: Close Mint using Anchor CPI
    let close_ix = Instruction::new_with_bytes(
        program_id,
        &mint_close_authority::instruction::Close {}.data(),
        mint_close_authority::accounts::Close {
            authority: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![close_ix], &[&payer], &payer.pubkey()).unwrap();

    // Verify mint no longer exists (lamports returned to authority)
    let mint_account = svm.get_account(&mint_keypair.pubkey());
    assert!(
        mint_account.is_none(),
        "Mint account should be closed"
    );

    svm.expire_blockhash();

    // Step 3: Create Mint with Close Authority again (re-use same keypair)
    let initialize_ix2 = Instruction::new_with_bytes(
        program_id,
        &mint_close_authority::instruction::Initialize {}.data(),
        mint_close_authority::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix2], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();

    // Verify mint exists again
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should exist after re-creation");
    assert!(
        !mint_account.data.is_empty(),
        "Mint should have data after re-creation"
    );

    svm.expire_blockhash();

    // Step 4: Close Mint directly using Token Extensions CloseAccount instruction
    let close_direct_ix = Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            anchor_lang::solana_program::instruction::AccountMeta::new(
                mint_keypair.pubkey(),
                false,
            ),
            anchor_lang::solana_program::instruction::AccountMeta::new(
                payer.pubkey(),
                false,
            ),
            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                payer.pubkey(),
                true,
            ),
        ],
        data: vec![9], // CloseAccount
    };
    send_transaction_from_instructions(&mut svm, vec![close_direct_ix], &[&payer], &payer.pubkey()).unwrap();

    // Verify mint is closed again
    let mint_account = svm.get_account(&mint_keypair.pubkey());
    assert!(
        mint_account.is_none(),
        "Mint account should be closed via direct Token Extensions instruction"
    );
}
