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
            create_token_extensions_account, create_token_extensions_mint,
            mint_tokens_to_token_extensions_account, transfer_checked_token_extensions,
            MintExtension, TOKEN_EXTENSIONS_PROGRAM_ID,
        },
        transfer_hook::{build_hook_accounts, get_hook_accounts_address, HookAccount},
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

fn associated_token_program_id() -> Pubkey {
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        .parse()
        .unwrap()
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = transfer_hook::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/transfer_hook.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_transfer_hook_account_data_as_seed() {
    let (mut svm, program_id, payer) = setup();
    let recipient = Keypair::new();
    let decimals: u8 = 9;

    // PDAs
    let (counter_pda, _) =
        Pubkey::find_program_address(&[b"counter", payer.pubkey().as_ref()], &program_id);

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

    let extra_account_meta_list =
        get_hook_accounts_address(&mint, &program_id);

    // Step 2: Create token accounts and mint tokens
    let amount: u64 = 100 * 10u64.pow(decimals as u32);
    let source_ata = create_token_extensions_account(
        &mut svm,
        &payer.pubkey(),
        &mint,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    let dest_ata = create_token_extensions_account(
        &mut svm,
        &recipient.pubkey(),
        &mint,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint,
        &source_ata,
        amount,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    // Step 3: Initialize ExtraAccountMetaList (also creates counter PDA)
    let init_extra_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_hook::instruction::InitializeExtraAccountMetaList {}.data(),
        transfer_hook::accounts::InitializeExtraAccountMetaList {
            payer: payer.pubkey(),
            extra_account_meta_list,
            mint,
            counter_account: counter_pda,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            associated_token_program: associated_token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![init_extra_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 4: Transfer with hook
    let transfer_amount: u64 = 1 * 10u64.pow(decimals as u32);
    let extra_accounts = build_hook_accounts(
        &mint,
        &program_id,
        &[HookAccount {
            pubkey: counter_pda,
            is_signer: false,
            is_writable: true,
        }],
    );
    transfer_checked_token_extensions(
        &mut svm,
        &source_ata,
        &mint,
        &dest_ata,
        &payer,
        transfer_amount,
        decimals,
        &extra_accounts,
    ).unwrap();
    svm.expire_blockhash();

    // Step 5: Try calling transfer_hook directly (should fail — not transferring)
    let direct_hook_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_hook::instruction::TransferHook { amount: 1 }.data(),
        transfer_hook::accounts::TransferHook {
            source_token: source_ata,
            mint,
            destination_token: dest_ata,
            owner: payer.pubkey(),
            extra_account_meta_list,
            counter_account: counter_pda,
        }
        .to_account_metas(None),
    );
    let result = send_transaction_from_instructions(&mut svm, vec![direct_hook_ix], &[&payer], &payer.pubkey());
    assert!(
        result.is_err(),
        "Calling transfer_hook directly should fail because token is not transferring"
    );
}
