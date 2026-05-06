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
        create_wallet, send_transaction_from_instructions,
        token_extensions::{create_token_extensions_mint, TOKEN_EXTENSIONS_PROGRAM_ID},
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

/// SetAuthority instruction for Token Extensions (instruction 6).
fn set_authority_instruction(
    account: &Pubkey,
    current_authority: &Pubkey,
    new_authority: Option<&Pubkey>,
    authority_type: u8,
) -> Instruction {
    let mut data = vec![6u8, authority_type];
    match new_authority {
        Some(new_auth) => {
            data.push(1); // COption::Some
            data.extend_from_slice(new_auth.as_ref());
        }
        None => {
            data.push(0); // COption::None
            data.extend_from_slice(&[0u8; 32]);
        }
    }
    Instruction {
        program_id: TOKEN_EXTENSIONS_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*account, false),
            AccountMeta::new_readonly(*current_authority, true),
        ],
        data,
    }
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = immutable_owner::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/immutable_owner.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_create_token_account_with_immutable_owner() {
    let (mut svm, program_id, payer) = setup();
    let token_keypair = Keypair::new();

    // Step 1: Create a Token Extensions mint with 2 decimals (no extensions needed on mint)
    let mint = create_token_extensions_mint(&mut svm, &payer, 2, None, &[]).unwrap();
    svm.expire_blockhash();

    // Step 2: Call program to create token account with ImmutableOwner extension
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &immutable_owner::instruction::Initialize {}.data(),
        immutable_owner::accounts::Initialize {
            payer: payer.pubkey(),
            token_account: token_keypair.pubkey(),
            mint_account: mint,
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &token_keypair], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Verify token account was created
    let token_data = svm
        .get_account(&token_keypair.pubkey())
        .expect("Token account should exist");
    assert!(
        token_data.data.len() > 165,
        "Token account should have extension data (size > 165, got {})",
        token_data.data.len()
    );

    // Step 3: Attempt to change the account owner — should fail due to immutable owner
    let new_owner = Keypair::new();
    let set_authority_ix = set_authority_instruction(
        &token_keypair.pubkey(),
        &payer.pubkey(),
        Some(&new_owner.pubkey()),
        2, // AuthorityType::AccountOwner
    );
    let result = send_transaction_from_instructions(&mut svm, vec![set_authority_ix], &[&payer], &payer.pubkey());
    assert!(
        result.is_err(),
        "Setting a new owner should fail due to ImmutableOwner extension"
    );
}
