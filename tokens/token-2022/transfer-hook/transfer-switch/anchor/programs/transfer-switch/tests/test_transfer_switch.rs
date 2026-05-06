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
    let program_id = transfer_switch::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/transfer_switch.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_transfer_switch() {
    let (mut svm, program_id, payer) = setup();
    let sender = create_wallet(&mut svm, 10_000_000_000).unwrap();
    let recipient = Keypair::new();
    let decimals: u8 = 9;

    // Derive PDAs
    let (admin_config, _) = Pubkey::find_program_address(&[b"admin-config"], &program_id);
    let (sender_switch, _) =
        Pubkey::find_program_address(&[sender.pubkey().as_ref()], &program_id);

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
        &sender.pubkey(),
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

    // Step 3: Configure admin
    let configure_admin_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_switch::instruction::ConfigureAdmin {}.data(),
        transfer_switch::accounts::ConfigureAdmin {
            admin: payer.pubkey(),
            new_admin: payer.pubkey(),
            admin_config,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![configure_admin_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 4: Initialize extra account metas list
    let init_extra_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_switch::instruction::InitializeExtraAccountMetasList {}.data(),
        transfer_switch::accounts::InitializeExtraAccountMetas {
            payer: payer.pubkey(),
            token_mint: mint,
            extra_account_metas_list: extra_account_meta_list,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![init_extra_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 5: Turn transfers OFF for sender
    let switch_off_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_switch::instruction::Switch { on: false }.data(),
        transfer_switch::accounts::Switch {
            admin: payer.pubkey(),
            wallet: sender.pubkey(),
            admin_config,
            wallet_switch: sender_switch,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![switch_off_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 6: Try transfer — should FAIL (switch is off)
    let transfer_amount: u64 = 1 * 10u64.pow(decimals as u32);
    let extra_accounts = build_hook_accounts(
        &mint,
        &program_id,
        &[HookAccount {
            pubkey: sender_switch,
            is_signer: false,
            is_writable: false,
        }],
    );
    let result = transfer_checked_token_extensions(
        &mut svm,
        &source_ata,
        &mint,
        &dest_ata,
        &sender,
        transfer_amount,
        decimals,
        &extra_accounts,
    );
    assert!(
        result.is_err(),
        "Transfer should fail when switch is off"
    );
    svm.expire_blockhash();

    // Step 7: Turn transfers ON for sender
    let switch_on_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_switch::instruction::Switch { on: true }.data(),
        transfer_switch::accounts::Switch {
            admin: payer.pubkey(),
            wallet: sender.pubkey(),
            admin_config,
            wallet_switch: sender_switch,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![switch_on_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 8: Transfer — should SUCCEED (switch is on)
    transfer_checked_token_extensions(
        &mut svm,
        &source_ata,
        &mint,
        &dest_ata,
        &sender,
        transfer_amount,
        decimals,
        &extra_accounts,
    ).unwrap();
}
