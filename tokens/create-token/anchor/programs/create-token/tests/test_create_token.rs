use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

fn metadata_program_id() -> Pubkey {
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        .parse()
        .unwrap()
}

fn token_program_id() -> Pubkey {
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        .parse()
        .unwrap()
}

fn rent_sysvar_id() -> Pubkey {
    "SysvarRent111111111111111111111111111111111"
        .parse()
        .unwrap()
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = create_token::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/create_token.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let metadata_bytes = include_bytes!("../../../tests/fixtures/mpl_token_metadata.so");
    svm.add_program(metadata_program_id(), metadata_bytes)
        .unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

fn derive_metadata_pda(mint: &Pubkey) -> Pubkey {
    let metadata_pid = metadata_program_id();
    let (pda, _bump) = Pubkey::find_program_address(
        &[b"metadata", metadata_pid.as_ref(), mint.as_ref()],
        &metadata_pid,
    );
    pda
}

#[test]
fn test_create_spl_token() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();
    let metadata_account = derive_metadata_pda(&mint_keypair.pubkey());

    let instruction = Instruction::new_with_bytes(
        program_id,
        &create_token::instruction::CreateTokenMint {
            _token_decimals: 9,
            token_name: "Solana Gold".to_string(),
            token_symbol: "GOLDSOL".to_string(),
            token_uri: "https://example.com/token.json".to_string(),
        }
        .data(),
        create_token::accounts::CreateTokenMint {
            payer: payer.pubkey(),
            metadata_account,
            mint_account: mint_keypair.pubkey(),
            token_metadata_program: metadata_program_id(),
            token_program: token_program_id(),
            system_program: system_program::id(),
            rent: rent_sysvar_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![instruction],
        &[&payer, &mint_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify the mint account exists
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should exist");
    assert!(!mint_account.data.is_empty(), "Mint account should have data");

    // Verify the metadata account was created
    let meta_account = svm
        .get_account(&metadata_account)
        .expect("Metadata account should exist");
    assert!(
        !meta_account.data.is_empty(),
        "Metadata account should have data"
    );
}

#[test]
fn test_create_nft() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();
    let metadata_account = derive_metadata_pda(&mint_keypair.pubkey());

    let instruction = Instruction::new_with_bytes(
        program_id,
        &create_token::instruction::CreateTokenMint {
            _token_decimals: 0,
            token_name: "Solana Gold".to_string(),
            token_symbol: "GOLDSOL".to_string(),
            token_uri: "https://example.com/nft.json".to_string(),
        }
        .data(),
        create_token::accounts::CreateTokenMint {
            payer: payer.pubkey(),
            metadata_account,
            mint_account: mint_keypair.pubkey(),
            token_metadata_program: metadata_program_id(),
            token_program: token_program_id(),
            system_program: system_program::id(),
            rent: rent_sysvar_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![instruction],
        &[&payer, &mint_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify the mint account exists
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should exist");
    assert!(!mint_account.data.is_empty(), "Mint account should have data");
}
