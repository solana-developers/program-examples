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

fn token_program_id() -> Pubkey {
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        .parse()
        .unwrap()
}

fn ata_program_id() -> Pubkey {
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        .parse()
        .unwrap()
}

fn metadata_program_id() -> Pubkey {
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        .parse()
        .unwrap()
}

fn rent_sysvar_id() -> Pubkey {
    "SysvarRent111111111111111111111111111111111"
        .parse()
        .unwrap()
}

fn derive_ata(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    let (ata, _bump) = Pubkey::find_program_address(
        &[wallet.as_ref(), token_program_id().as_ref(), mint.as_ref()],
        &ata_program_id(),
    );
    ata
}

fn derive_metadata_pda(mint: &Pubkey) -> Pubkey {
    let metadata_pid = metadata_program_id();
    let (pda, _bump) = Pubkey::find_program_address(
        &[b"metadata", metadata_pid.as_ref(), mint.as_ref()],
        &metadata_pid,
    );
    pda
}

fn derive_edition_pda(mint: &Pubkey) -> Pubkey {
    let metadata_pid = metadata_program_id();
    let (pda, _bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            metadata_pid.as_ref(),
            mint.as_ref(),
            b"edition",
        ],
        &metadata_pid,
    );
    pda
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = nft_minter::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/nft_minter.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let metadata_bytes = include_bytes!("../../../tests/fixtures/mpl_token_metadata.so");
    svm.add_program(metadata_program_id(), metadata_bytes)
        .unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_mint_nft() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();

    let metadata_account = derive_metadata_pda(&mint_keypair.pubkey());
    let edition_account = derive_edition_pda(&mint_keypair.pubkey());
    let associated_token_account = derive_ata(&payer.pubkey(), &mint_keypair.pubkey());

    let instruction = Instruction::new_with_bytes(
        program_id,
        &nft_minter::instruction::MintNft {
            nft_name: "Homer NFT".to_string(),
            nft_symbol: "HOMR".to_string(),
            nft_uri: "https://example.com/nft.json".to_string(),
        }
        .data(),
        nft_minter::accounts::CreateToken {
            payer: payer.pubkey(),
            metadata_account,
            edition_account,
            mint_account: mint_keypair.pubkey(),
            associated_token_account,
            token_program: token_program_id(),
            token_metadata_program: metadata_program_id(),
            associated_token_program: ata_program_id(),
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

    // Verify the mint account exists (NFT = 0 decimals)
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint account should exist");
    assert!(!mint_account.data.is_empty());

    // Verify the metadata account was created
    let meta_account = svm
        .get_account(&metadata_account)
        .expect("Metadata account should exist");
    assert!(!meta_account.data.is_empty());

    // Verify the edition account was created
    let edition = svm
        .get_account(&edition_account)
        .expect("Edition account should exist");
    assert!(!edition.data.is_empty());

    // Verify 1 NFT was minted to the associated token account
    let balance = get_token_account_balance(&svm, &associated_token_account).unwrap();
    assert_eq!(balance, 1, "Should have exactly 1 NFT");
}
