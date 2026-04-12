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
            mint_tokens_to_token_extensions_account, TOKEN_EXTENSIONS_PROGRAM_ID,
        },
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

/// Create a Token Extensions token account (165 bytes, no extra extensions).
/// Uses explicit keypair — not an ATA — so we can inspect account state bytes.
fn create_token_account_instruction(
    payer: &Pubkey,
    token_account: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Vec<Instruction> {
    let rent_sysvar: Pubkey = "SysvarRent111111111111111111111111111111111"
        .parse()
        .unwrap();
    let create_ix = anchor_lang::solana_program::system_instruction::create_account(
        payer,
        token_account,
        3_000_000,
        165,
        &TOKEN_EXTENSIONS_PROGRAM_ID,
    );
    let init_ix = Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*token_account, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(*owner, false),
            AccountMeta::new_readonly(rent_sysvar, false),
        ],
        data: vec![1], // InitializeAccount
    };
    vec![create_ix, init_ix]
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = default_account_state::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/default_account_state.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_default_account_state() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();

    // Step 1: Initialize mint with DefaultAccountState extension (frozen)
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &default_account_state::instruction::Initialize {}.data(),
        default_account_state::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Verify mint exists
    let mint_account = svm.get_account(&mint_keypair.pubkey()).unwrap();
    assert!(!mint_account.data.is_empty(), "Mint should have data");

    // Step 2: Create a token account (it will be frozen by default due to DefaultAccountState extension)
    let token1 = Keypair::new();
    let create_token1_ixs = create_token_account_instruction(
        &payer.pubkey(),
        &token1.pubkey(),
        &mint_keypair.pubkey(),
        &payer.pubkey(),
    );
    send_transaction_from_instructions(&mut svm, create_token1_ixs, &[&payer, &token1], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Verify token account state is frozen (byte 108 = account state: 0=uninitialized, 1=initialized, 2=frozen)
    let token1_data = svm.get_account(&token1.pubkey()).unwrap();
    assert_eq!(
        token1_data.data[108], 2,
        "Token account should be frozen (state=2)"
    );

    // Step 3: Attempt to mint to the frozen account — should fail
    let result = mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint_keypair.pubkey(),
        &token1.pubkey(),
        1,
        &payer,
    );
    assert!(
        result.is_err(),
        "Minting to a frozen account should fail"
    );
    svm.expire_blockhash();

    // Step 4: Update default state to Initialized
    let update_ix = Instruction::new_with_bytes(
        program_id,
        &default_account_state::instruction::UpdateDefaultState {
            account_state: default_account_state::AnchorAccountState::Initialized,
        }
        .data(),
        default_account_state::accounts::UpdateDefaultState {
            freeze_authority: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![update_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 5: Create a new token account — should be initialized (not frozen) now
    let token2 = Keypair::new();
    let create_token2_ixs = create_token_account_instruction(
        &payer.pubkey(),
        &token2.pubkey(),
        &mint_keypair.pubkey(),
        &payer.pubkey(),
    );
    send_transaction_from_instructions(&mut svm, create_token2_ixs, &[&payer, &token2], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Verify token2 is initialized (not frozen)
    let token2_data = svm.get_account(&token2.pubkey()).unwrap();
    assert_eq!(
        token2_data.data[108], 1,
        "Token account should be initialized (state=1)"
    );

    // Step 6: Mint to the new account — should succeed
    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint_keypair.pubkey(),
        &token2.pubkey(),
        1,
        &payer,
    ).unwrap();

    assert_token_account_balance(&svm, &token2.pubkey(), 1, "Should have minted 1 token");
}
