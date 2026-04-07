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

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = transfer_hook::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/transfer_hook.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_whitelist_transfer_hook() {
    let (mut svm, program_id, payer) = setup();
    let recipient = Keypair::new();
    let decimals: u8 = 9;

    // Derive PDAs
    let (white_list_pda, _) = Pubkey::find_program_address(&[b"white_list"], &program_id);

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

    // Step 3: Initialize ExtraAccountMetaList (also creates whitelist)
    let init_extra_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_hook::instruction::InitializeExtraAccountMetaList {}.data(),
        transfer_hook::accounts::InitializeExtraAccountMetaList {
            payer: payer.pubkey(),
            extra_account_meta_list,
            mint,
            system_program: system_program::id(),
            white_list: white_list_pda,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![init_extra_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 4: Add destination token account to whitelist
    let add_to_whitelist_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_hook::instruction::AddToWhitelist {}.data(),
        transfer_hook::accounts::AddToWhiteList {
            new_account: dest_ata,
            white_list: white_list_pda,
            signer: payer.pubkey(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![add_to_whitelist_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 5: Transfer — should succeed (destination is whitelisted)
    let transfer_amount: u64 = 1 * 10u64.pow(decimals as u32);
    let extra_accounts = build_hook_accounts(
        &mint,
        &program_id,
        &[HookAccount {
            pubkey: white_list_pda,
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
}
