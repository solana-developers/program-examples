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

fn memo_program_id() -> Pubkey {
    "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
        .parse()
        .unwrap()
}

/// Create a Token Extensions token account (165 bytes, no extra extensions).
/// Uses explicit keypair — not an ATA — because the test needs multiple
/// source accounts for the same owner+mint.
fn create_token_account_instructions(
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

/// Transfer instruction for Token Extensions (instruction 3).
fn transfer_instruction(
    source: &Pubkey,
    dest: &Pubkey,
    authority: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![3u8];
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*source, false),
            AccountMeta::new(*dest, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data,
    }
}

/// Memo instruction: just the memo text as bytes.
fn memo_instruction(memo_text: &str, signers: &[&Pubkey]) -> Instruction {
    let accounts: Vec<AccountMeta> = signers
        .iter()
        .map(|s| AccountMeta::new_readonly(**s, true))
        .collect();
    Instruction {
        program_id: memo_program_id(),
        accounts,
        data: memo_text.as_bytes().to_vec(),
    }
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = memo_transfer::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/memo_transfer.so");
    svm.add_program(program_id, program_bytes).unwrap();

    // Load SPL Memo program (needed for memo instructions)
    let memo_program_bytes = include_bytes!("../../../tests/fixtures/spl_memo.so");
    svm.add_program(memo_program_id(), memo_program_bytes)
        .unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_memo_transfer() {
    let (mut svm, program_id, payer) = setup();
    let token_keypair = Keypair::new();

    // Step 1: Create a standard Token Extensions mint (no extensions on the mint)
    let mint = create_token_extensions_mint(&mut svm, &payer, 2, None, &[]).unwrap();
    svm.expire_blockhash();

    // Step 2: Create token account with RequiredMemo extension via program
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &memo_transfer::instruction::Initialize {}.data(),
        memo_transfer::accounts::Initialize {
            payer: payer.pubkey(),
            token_account: token_keypair.pubkey(),
            mint_account: mint,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &token_keypair], &payer.pubkey()).unwrap();

    // Verify token account exists
    let token_account = svm
        .get_account(&token_keypair.pubkey())
        .expect("Token account should exist");
    assert!(
        !token_account.data.is_empty(),
        "Token account should have data"
    );

    svm.expire_blockhash();

    // Step 3: Create a source token account and mint tokens to it
    let source_keypair = Keypair::new();
    let create_source_ixs = create_token_account_instructions(
        &payer.pubkey(),
        &source_keypair.pubkey(),
        &mint,
        &payer.pubkey(),
    );
    send_transaction_from_instructions(&mut svm, create_source_ixs, &[&payer, &source_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint,
        &source_keypair.pubkey(),
        100,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    // Step 4: Transfer without memo — should fail
    let transfer_ix = transfer_instruction(
        &source_keypair.pubkey(),
        &token_keypair.pubkey(),
        &payer.pubkey(),
        1,
    );
    let result = send_transaction_from_instructions(&mut svm, vec![transfer_ix], &[&payer], &payer.pubkey());
    assert!(
        result.is_err(),
        "Transfer without memo should fail"
    );
    svm.expire_blockhash();

    // Step 5: Transfer with memo — should succeed
    let memo_ix = memo_instruction("hello, world", &[&payer.pubkey()]);
    let transfer_ix = transfer_instruction(
        &source_keypair.pubkey(),
        &token_keypair.pubkey(),
        &payer.pubkey(),
        1,
    );
    send_transaction_from_instructions(&mut svm, vec![memo_ix, transfer_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    assert_token_account_balance(&svm, &token_keypair.pubkey(), 1, "Should have 1 token after transfer with memo");

    // Step 6: Disable RequiredMemo extension
    let disable_ix = Instruction::new_with_bytes(
        program_id,
        &memo_transfer::instruction::Disable {}.data(),
        memo_transfer::accounts::Disable {
            owner: payer.pubkey(),
            token_account: token_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![disable_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 7: Transfer without memo should now succeed (memo disabled)
    let source2_keypair = Keypair::new();
    let create_source2_ixs = create_token_account_instructions(
        &payer.pubkey(),
        &source2_keypair.pubkey(),
        &mint,
        &payer.pubkey(),
    );
    send_transaction_from_instructions(&mut svm, create_source2_ixs, &[&payer, &source2_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint,
        &source2_keypair.pubkey(),
        100,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    let transfer_ix2 = transfer_instruction(
        &source2_keypair.pubkey(),
        &token_keypair.pubkey(),
        &payer.pubkey(),
        1,
    );
    send_transaction_from_instructions(&mut svm, vec![transfer_ix2], &[&payer], &payer.pubkey()).unwrap();

    assert_token_account_balance(
        &svm,
        &token_keypair.pubkey(),
        2,
        "Should have 2 tokens after transfer without memo (memo disabled)",
    );
}
