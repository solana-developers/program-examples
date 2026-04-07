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

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = permanent_delegate::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/permanent_delegate.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

/// Create a Token Extensions token account (CreateAccount + InitializeAccount3).
/// This creates a non-ATA token account with explicit keypair, which kite doesn't provide.
fn create_token_account_instructions(
    payer: &Pubkey,
    account: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Vec<Instruction> {
    let space: u64 = 200;
    let lamports: u64 = 3_000_000;
    let create_account_ix = anchor_lang::solana_program::system_instruction::create_account(
        payer, account, lamports, space, &TOKEN_EXTENSIONS_PROGRAM_ID,
    );
    // InitializeAccount3 (instruction 18): [18, owner(32)]
    let mut init_data = vec![18u8];
    init_data.extend_from_slice(owner.as_ref());
    let init_account_ix = Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*account, false),
            AccountMeta::new_readonly(*mint, false),
        ],
        data: init_data,
    };
    vec![create_account_ix, init_account_ix]
}

/// BurnChecked instruction for Token Extensions (instruction 15).
fn burn_checked_ix(
    account: &Pubkey,
    mint: &Pubkey,
    authority: &Pubkey,
    amount: u64,
    decimals: u8,
) -> Instruction {
    let mut data = vec![15u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(decimals);
    Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*account, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data,
    }
}

#[test]
fn test_create_mint_with_permanent_delegate_and_burn() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();

    // Step 1: Create mint with PermanentDelegate extension via program
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &permanent_delegate::instruction::Initialize {}.data(),
        permanent_delegate::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 2: Create a token account owned by a random keypair
    let random_owner = Keypair::new();
    let token_account = Keypair::new();
    let create_ata_ixs = create_token_account_instructions(
        &payer.pubkey(),
        &token_account.pubkey(),
        &mint_keypair.pubkey(),
        &random_owner.pubkey(),
    );
    send_transaction_from_instructions(&mut svm, create_ata_ixs, &[&payer, &token_account], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 3: Mint 100 tokens to the token account
    mint_tokens_to_token_extensions_account(
        &mut svm,
        &mint_keypair.pubkey(),
        &token_account.pubkey(),
        100,
        &payer,
    ).unwrap();
    svm.expire_blockhash();

    // Step 4: Burn all 100 tokens using the permanent delegate (payer)
    let burn_ix = burn_checked_ix(
        &token_account.pubkey(),
        &mint_keypair.pubkey(),
        &payer.pubkey(),
        100,
        2, // decimals
    );
    send_transaction_from_instructions(&mut svm, vec![burn_ix], &[&payer], &payer.pubkey()).unwrap();

    assert_token_account_balance(&svm, &token_account.pubkey(), 0, "Token account balance should be 0 after burn");
}
