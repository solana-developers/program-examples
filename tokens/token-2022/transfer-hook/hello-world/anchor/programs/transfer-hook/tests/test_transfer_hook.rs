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
        transfer_hook::{build_hook_accounts, get_hook_accounts_address},
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
fn test_transfer_hook_hello_world() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();
    let recipient = Keypair::new();
    let ata_program = associated_token_program_id();
    let decimals: u8 = 2;

    // ExtraAccountMetaList PDA
    let extra_account_meta_list =
        get_hook_accounts_address(&mint_keypair.pubkey(), &program_id);

    // Step 1: Create mint with transfer hook extension pointing to our program
    // (uses the program's own Initialize instruction, not kite, since it sets up
    // the mint with the program as the hook authority)
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_hook::instruction::Initialize {
            _decimals: decimals,
        }
        .data(),
        transfer_hook::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 2: Create token accounts and mint tokens
    let amount: u64 = 100 * 10u64.pow(decimals as u32);
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

    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint_keypair.pubkey(),
        &source_ata,
        amount,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    // Step 3: Create ExtraAccountMetaList account
    let init_extra_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_hook::instruction::InitializeExtraAccountMetaList {}.data(),
        transfer_hook::accounts::InitializeExtraAccountMetaList {
            payer: payer.pubkey(),
            extra_account_meta_list,
            mint: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            associated_token_program: ata_program,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![init_extra_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 4: Transfer with transfer hook
    let transfer_amount: u64 = 1 * 10u64.pow(decimals as u32);
    let extra_accounts = build_hook_accounts(
        &mint_keypair.pubkey(),
        &program_id,
        &[], // hello-world hook has no user-defined extra accounts
    );
    transfer_checked_token_extensions(
        &mut svm,
        &source_ata,
        &mint_keypair.pubkey(),
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
        &transfer_hook::instruction::TransferHook { _amount: 1 }.data(),
        transfer_hook::accounts::TransferHook {
            source_token: source_ata,
            mint: mint_keypair.pubkey(),
            destination_token: dest_ata,
            owner: payer.pubkey(),
            extra_account_meta_list,
        }
        .to_account_metas(None),
    );
    let result = send_transaction_from_instructions(&mut svm, vec![direct_hook_ix], &[&payer], &payer.pubkey());
    assert!(
        result.is_err(),
        "Calling transfer_hook directly should fail because token is not transferring"
    );
}
