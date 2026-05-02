use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_kite::{
        assert_token_account_balance, create_wallet, send_transaction_from_instructions,
        token_extensions::{
            create_token_extensions_mint, mint_tokens_to_token_extensions_account,
            TOKEN_EXTENSIONS_PROGRAM_ID,
        },
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = cpi_guard::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/cpi_guard.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

/// Create a basic Token Extensions token account (165 bytes, no extensions).
/// Uses explicit keypair — kite's ATA creation won't work here because
/// we need to reallocate and add the CPI Guard extension later.
fn create_basic_token_account_instructions(
    payer: &Pubkey,
    token_account: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Vec<Instruction> {
    let rent_sysvar: Pubkey = "SysvarRent111111111111111111111111111111111"
        .parse()
        .unwrap();
    let create_account_ix = anchor_lang::solana_program::system_instruction::create_account(
        payer,
        token_account,
        3_000_000,
        165,
        &TOKEN_EXTENSIONS_PROGRAM_ID,
    );
    let init_account_ix = Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*token_account, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(*owner, false),
            AccountMeta::new_readonly(rent_sysvar, false),
        ],
        data: vec![1], // InitializeAccount
    };
    vec![create_account_ix, init_account_ix]
}

/// Reallocate instruction (instruction 29) to add extension types to a token account.
fn reallocate_instruction(
    token_account: &Pubkey,
    payer: &Pubkey,
    owner: &Pubkey,
    extension_types: &[u16],
) -> Instruction {
    let mut data = vec![29u8];
    for et in extension_types {
        data.extend_from_slice(&et.to_le_bytes());
    }
    Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*token_account, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(*owner, true),
        ],
        data,
    }
}

/// EnableCpiGuard instruction (instruction 34, sub-instruction 0).
fn enable_cpi_guard_instruction(token_account: &Pubkey, owner: &Pubkey) -> Instruction {
    Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*token_account, false),
            AccountMeta::new_readonly(*owner, true),
        ],
        data: vec![34, 0],
    }
}

/// DisableCpiGuard instruction (instruction 34, sub-instruction 1).
fn disable_cpi_guard_instruction(token_account: &Pubkey, owner: &Pubkey) -> Instruction {
    Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*token_account, false),
            AccountMeta::new_readonly(*owner, true),
        ],
        data: vec![34, 1],
    }
}

#[test]
fn test_cpi_guard_prevents_transfer_then_allows_after_disable() {
    let (mut svm, program_id, payer) = setup();
    let token_keypair = Keypair::new();

    // Step 1: Create a Token Extensions mint (no extensions needed on the mint itself)
    let mint = create_token_extensions_mint(&mut svm, &payer, 2, None, &[]).unwrap();
    svm.expire_blockhash();

    // Step 2: Create basic token account (needs explicit keypair for reallocate flow)
    let token_ixs = create_basic_token_account_instructions(
        &payer.pubkey(),
        &token_keypair.pubkey(),
        &mint,
        &payer.pubkey(),
    );
    send_transaction_from_instructions(&mut svm, token_ixs, &[&payer, &token_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 3: Reallocate to add CPI Guard extension space
    let cpi_guard_extension_type: u16 = 11; // ExtensionType::CpiGuard
    let reallocate_ix = reallocate_instruction(
        &token_keypair.pubkey(),
        &payer.pubkey(),
        &payer.pubkey(),
        &[cpi_guard_extension_type],
    );
    send_transaction_from_instructions(&mut svm, vec![reallocate_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 4: Enable CPI Guard
    let enable_ix = enable_cpi_guard_instruction(&token_keypair.pubkey(), &payer.pubkey());
    send_transaction_from_instructions(&mut svm, vec![enable_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 5: Mint 1 token to the token account
    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint,
        &token_keypair.pubkey(),
        1,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    // Step 6: Try CPI transfer — should fail because CPI Guard is enabled
    let (recipient_token_account, _bump) =
        Pubkey::find_program_address(&[b"pda"], &program_id);

    let transfer_ix = Instruction::new_with_bytes(
        program_id,
        &cpi_guard::instruction::CpiTransfer {}.data(),
        cpi_guard::accounts::CpiTransfer {
            sender: payer.pubkey(),
            sender_token_account: token_keypair.pubkey(),
            recipient_token_account,
            mint_account: mint,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    let result = send_transaction_from_instructions(&mut svm, vec![transfer_ix], &[&payer], &payer.pubkey());
    assert!(
        result.is_err(),
        "Transfer should fail when CPI Guard is enabled"
    );
    svm.expire_blockhash();

    // Step 7: Disable CPI Guard
    let disable_ix = disable_cpi_guard_instruction(&token_keypair.pubkey(), &payer.pubkey());
    send_transaction_from_instructions(&mut svm, vec![disable_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 8: Transfer should now succeed
    let transfer_ix2 = Instruction::new_with_bytes(
        program_id,
        &cpi_guard::instruction::CpiTransfer {}.data(),
        cpi_guard::accounts::CpiTransfer {
            sender: payer.pubkey(),
            sender_token_account: token_keypair.pubkey(),
            recipient_token_account,
            mint_account: mint,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(&mut svm, vec![transfer_ix2], &[&payer], &payer.pubkey()).unwrap();

    assert_token_account_balance(&svm, &recipient_token_account, 1, "Recipient should have 1 token");
}
