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
            create_token_extensions_mint, MintExtension, TOKEN_EXTENSIONS_PROGRAM_ID,
        },
        transfer_hook::get_hook_accounts_address,
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = transfer_hook::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/transfer_hook.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_initialize_extra_account_meta_list() {
    let (mut svm, program_id, payer) = setup();
    let decimals: u8 = 9;

    // Step 1: Create mint with TransferHook extension
    let mint = create_token_extensions_mint(
        &mut svm,
        &payer,
        decimals,
        None,
        &[MintExtension::TransferHook {
            program_id: program_id,
        }],
    )
    .unwrap();
    svm.expire_blockhash();

    // PDAs
    let extra_account_meta_list =
        get_hook_accounts_address(&mint, &program_id);
    let (counter_pda, _) = Pubkey::find_program_address(&[b"counter"], &program_id);

    // Step 2: Initialize ExtraAccountMetaList (also creates counter PDA)
    let init_extra_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_hook::instruction::InitializeExtraAccountMetaList {}.data(),
        transfer_hook::accounts::InitializeExtraAccountMetaList {
            payer: payer.pubkey(),
            extra_account_meta_list,
            mint,
            counter_account: counter_pda,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![init_extra_ix], &[&payer], &payer.pubkey()).unwrap();

    // Verify the ExtraAccountMetaList account was created
    let account = svm.get_account(&extra_account_meta_list);
    assert!(
        account.is_some(),
        "ExtraAccountMetaList account should exist after initialization"
    );

    // Verify the counter account was created
    let counter = svm.get_account(&counter_pda);
    assert!(
        counter.is_some(),
        "Counter account should exist after initialization"
    );
}
