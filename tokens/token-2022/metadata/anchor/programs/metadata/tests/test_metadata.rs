use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_kite::{
        create_wallet, send_transaction_from_instructions,
        token_extensions::TOKEN_EXTENSIONS_PROGRAM_ID,
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = metadata::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/metadata.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_metadata_full_flow() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();

    // Step 1: Initialize mint with MetadataPointer and TokenMetadata extensions
    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &metadata::instruction::Initialize {
            args: metadata::instructions::TokenMetadataArgs {
                name: "OPOS".to_string(),
                symbol: "OPOS".to_string(),
                uri: "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json".to_string(),
            },
        }
        .data(),
        metadata::accounts::Initialize {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![initialize_ix], &[&payer, &mint_keypair], &payer.pubkey()).unwrap();

    // Verify mint exists
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should exist");
    assert!(!mint_account.data.is_empty(), "Mint should have data");

    svm.expire_blockhash();

    // Step 2: Update existing metadata field (name)
    let update_name_ix = Instruction::new_with_bytes(
        program_id,
        &metadata::instruction::UpdateField {
            args: metadata::instructions::UpdateFieldArgs {
                field: metadata::instructions::AnchorField::Name,
                value: "Solana".to_string(),
            },
        }
        .data(),
        metadata::accounts::UpdateField {
            authority: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![update_name_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 3: Add custom field
    let add_custom_field_ix = Instruction::new_with_bytes(
        program_id,
        &metadata::instruction::UpdateField {
            args: metadata::instructions::UpdateFieldArgs {
                field: metadata::instructions::AnchorField::Key("color".to_string()),
                value: "red".to_string(),
            },
        }
        .data(),
        metadata::accounts::UpdateField {
            authority: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![add_custom_field_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 4: Remove custom field
    let remove_key_ix = Instruction::new_with_bytes(
        program_id,
        &metadata::instruction::RemoveKey {
            key: "color".to_string(),
        }
        .data(),
        metadata::accounts::RemoveKey {
            update_authority: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![remove_key_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 5: Update authority to None
    let update_authority_ix = Instruction::new_with_bytes(
        program_id,
        &metadata::instruction::UpdateAuthority {}.data(),
        metadata::accounts::UpdateAuthority {
            current_authority: payer.pubkey(),
            new_authority: None,
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![update_authority_ix], &[&payer], &payer.pubkey()).unwrap();
    svm.expire_blockhash();

    // Step 6: Emit metadata (verify it doesn't fail)
    let emit_ix = Instruction::new_with_bytes(
        program_id,
        &metadata::instruction::Emit {}.data(),
        metadata::accounts::Emit {
            mint_account: mint_keypair.pubkey(),
            token_program: TOKEN_EXTENSIONS_PROGRAM_ID,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![emit_ix], &[&payer], &payer.pubkey()).unwrap();

    // Verify mint still exists after all operations
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should still exist after all metadata operations");
    assert!(
        !mint_account.data.is_empty(),
        "Mint should still have data"
    );
}
