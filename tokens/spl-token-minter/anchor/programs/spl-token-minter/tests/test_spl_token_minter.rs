use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{
        create_wallet, get_token_account_balance, send_transaction_from_instructions,
    },
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

fn associated_token_program_id() -> Pubkey {
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        .parse()
        .unwrap()
}

fn rent_sysvar_id() -> Pubkey {
    "SysvarRent111111111111111111111111111111111"
        .parse()
        .unwrap()
}

fn derive_metadata_pda(mint: &Pubkey) -> Pubkey {
    let metadata_pid = metadata_program_id();
    let (pda, _bump) = Pubkey::find_program_address(
        &[b"metadata", metadata_pid.as_ref(), mint.as_ref()],
        &metadata_pid,
    );
    pda
}

fn derive_ata(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    let (ata, _bump) = Pubkey::find_program_address(
        &[
            wallet.as_ref(),
            token_program_id().as_ref(),
            mint.as_ref(),
        ],
        &associated_token_program_id(),
    );
    ata
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = spl_token_minter::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/spl_token_minter.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let metadata_bytes = include_bytes!("../../../tests/fixtures/mpl_token_metadata.so");
    svm.add_program(metadata_program_id(), metadata_bytes)
        .unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_create_token() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();
    let metadata_account = derive_metadata_pda(&mint_keypair.pubkey());

    let create_ix = Instruction::new_with_bytes(
        program_id,
        &spl_token_minter::instruction::CreateToken {
            token_name: "Solana Gold".to_string(),
            token_symbol: "GOLDSOL".to_string(),
            token_uri: "https://example.com/token.json".to_string(),
        }
        .data(),
        spl_token_minter::accounts::CreateToken {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            metadata_account,
            token_program: token_program_id(),
            token_metadata_program: metadata_program_id(),
            system_program: system_program::id(),
            rent: rent_sysvar_id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![create_ix],
        &[&payer, &mint_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify mint created
    let mint = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint should exist");
    assert!(!mint.data.is_empty());

    // Verify metadata created
    let meta = svm
        .get_account(&metadata_account)
        .expect("Metadata should exist");
    assert!(!meta.data.is_empty());
}

#[test]
fn test_create_and_mint_tokens() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();
    let metadata_account = derive_metadata_pda(&mint_keypair.pubkey());

    // 1. Create token
    let create_ix = Instruction::new_with_bytes(
        program_id,
        &spl_token_minter::instruction::CreateToken {
            token_name: "Solana Gold".to_string(),
            token_symbol: "GOLDSOL".to_string(),
            token_uri: "https://example.com/token.json".to_string(),
        }
        .data(),
        spl_token_minter::accounts::CreateToken {
            payer: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            metadata_account,
            token_program: token_program_id(),
            token_metadata_program: metadata_program_id(),
            system_program: system_program::id(),
            rent: rent_sysvar_id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![create_ix],
        &[&payer, &mint_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // 2. Mint 100 tokens
    svm.expire_blockhash();
    let ata = derive_ata(&payer.pubkey(), &mint_keypair.pubkey());

    let mint_ix = Instruction::new_with_bytes(
        program_id,
        &spl_token_minter::instruction::MintToken { amount: 100 }.data(),
        spl_token_minter::accounts::MintToken {
            mint_authority: payer.pubkey(),
            recipient: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            associated_token_account: ata,
            token_program: token_program_id(),
            associated_token_program: associated_token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![mint_ix],
        &[&payer],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify: 100 * 10^9 = 100_000_000_000 tokens minted (9 decimals)
    let balance = get_token_account_balance(&svm, &ata).unwrap();
    assert_eq!(balance, 100_000_000_000, "Should have 100 tokens");
}
