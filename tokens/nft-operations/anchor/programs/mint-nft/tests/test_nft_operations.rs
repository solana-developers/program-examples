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

fn instructions_sysvar_id() -> Pubkey {
    "Sysvar1nstructions1111111111111111111111111"
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
    let program_id = mint_nft::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/mint_nft.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let metadata_bytes = include_bytes!("../../../tests/fixtures/mpl_token_metadata.so");
    svm.add_program(metadata_program_id(), metadata_bytes)
        .unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_create_collection() {
    let (mut svm, program_id, payer) = setup();
    let collection_keypair = Keypair::new();

    let (mint_authority, _) = Pubkey::find_program_address(&[b"authority"], &program_id);

    let metadata = derive_metadata_pda(&collection_keypair.pubkey());
    let master_edition = derive_edition_pda(&collection_keypair.pubkey());
    let destination = derive_ata(&payer.pubkey(), &collection_keypair.pubkey());

    let instruction = Instruction::new_with_bytes(
        program_id,
        &mint_nft::instruction::CreateCollection {}.data(),
        mint_nft::accounts::CreateCollection {
            user: payer.pubkey(),
            mint: collection_keypair.pubkey(),
            mint_authority,
            metadata,
            master_edition,
            destination,
            system_program: system_program::id(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            token_metadata_program: metadata_program_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![instruction],
        &[&payer, &collection_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify collection mint exists
    let mint_account = svm
        .get_account(&collection_keypair.pubkey())
        .expect("Collection mint should exist");
    assert!(!mint_account.data.is_empty());

    // Verify metadata exists
    let meta_account = svm
        .get_account(&metadata)
        .expect("Metadata should exist");
    assert!(!meta_account.data.is_empty());

    // Verify master edition exists
    let edition_account = svm
        .get_account(&master_edition)
        .expect("Master edition should exist");
    assert!(!edition_account.data.is_empty());

    // Verify 1 token was minted to destination
    let balance = get_token_account_balance(&svm, &destination).unwrap();
    assert_eq!(balance, 1, "Should have 1 collection token");
}

#[test]
fn test_mint_nft_to_collection() {
    let (mut svm, program_id, payer) = setup();

    let (mint_authority, _) = Pubkey::find_program_address(&[b"authority"], &program_id);

    // Step 1: Create the collection
    let collection_keypair = Keypair::new();
    let collection_metadata = derive_metadata_pda(&collection_keypair.pubkey());
    let collection_master_edition = derive_edition_pda(&collection_keypair.pubkey());
    let collection_destination = derive_ata(&payer.pubkey(), &collection_keypair.pubkey());

    let create_collection_ix = Instruction::new_with_bytes(
        program_id,
        &mint_nft::instruction::CreateCollection {}.data(),
        mint_nft::accounts::CreateCollection {
            user: payer.pubkey(),
            mint: collection_keypair.pubkey(),
            mint_authority,
            metadata: collection_metadata,
            master_edition: collection_master_edition,
            destination: collection_destination,
            system_program: system_program::id(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            token_metadata_program: metadata_program_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![create_collection_ix],
        &[&payer, &collection_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Step 2: Mint an NFT into the collection
    svm.expire_blockhash();
    let nft_keypair = Keypair::new();
    let nft_metadata = derive_metadata_pda(&nft_keypair.pubkey());
    let nft_master_edition = derive_edition_pda(&nft_keypair.pubkey());
    let nft_destination = derive_ata(&payer.pubkey(), &nft_keypair.pubkey());

    let mint_nft_ix = Instruction::new_with_bytes(
        program_id,
        &mint_nft::instruction::MintNft {}.data(),
        mint_nft::accounts::MintNFT {
            owner: payer.pubkey(),
            mint: nft_keypair.pubkey(),
            destination: nft_destination,
            metadata: nft_metadata,
            master_edition: nft_master_edition,
            mint_authority,
            collection_mint: collection_keypair.pubkey(),
            system_program: system_program::id(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            token_metadata_program: metadata_program_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![mint_nft_ix],
        &[&payer, &nft_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify NFT was minted
    let balance = get_token_account_balance(&svm, &nft_destination).unwrap();
    assert_eq!(balance, 1, "Should have 1 NFT");

    // Verify NFT metadata exists
    let nft_meta = svm
        .get_account(&nft_metadata)
        .expect("NFT metadata should exist");
    assert!(!nft_meta.data.is_empty());
}

#[test]
fn test_verify_collection() {
    let (mut svm, program_id, payer) = setup();

    let (mint_authority, _) = Pubkey::find_program_address(&[b"authority"], &program_id);

    // Step 1: Create collection
    let collection_keypair = Keypair::new();
    let collection_metadata = derive_metadata_pda(&collection_keypair.pubkey());
    let collection_master_edition = derive_edition_pda(&collection_keypair.pubkey());
    let collection_destination = derive_ata(&payer.pubkey(), &collection_keypair.pubkey());

    let create_collection_ix = Instruction::new_with_bytes(
        program_id,
        &mint_nft::instruction::CreateCollection {}.data(),
        mint_nft::accounts::CreateCollection {
            user: payer.pubkey(),
            mint: collection_keypair.pubkey(),
            mint_authority,
            metadata: collection_metadata,
            master_edition: collection_master_edition,
            destination: collection_destination,
            system_program: system_program::id(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            token_metadata_program: metadata_program_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![create_collection_ix],
        &[&payer, &collection_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Step 2: Mint NFT
    svm.expire_blockhash();
    let nft_keypair = Keypair::new();
    let nft_metadata = derive_metadata_pda(&nft_keypair.pubkey());
    let nft_master_edition = derive_edition_pda(&nft_keypair.pubkey());
    let nft_destination = derive_ata(&payer.pubkey(), &nft_keypair.pubkey());

    let mint_nft_ix = Instruction::new_with_bytes(
        program_id,
        &mint_nft::instruction::MintNft {}.data(),
        mint_nft::accounts::MintNFT {
            owner: payer.pubkey(),
            mint: nft_keypair.pubkey(),
            destination: nft_destination,
            metadata: nft_metadata,
            master_edition: nft_master_edition,
            mint_authority,
            collection_mint: collection_keypair.pubkey(),
            system_program: system_program::id(),
            token_program: token_program_id(),
            associated_token_program: ata_program_id(),
            token_metadata_program: metadata_program_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![mint_nft_ix],
        &[&payer, &nft_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Step 3: Verify collection
    svm.expire_blockhash();
    let verify_ix = Instruction::new_with_bytes(
        program_id,
        &mint_nft::instruction::VerifyCollection {}.data(),
        mint_nft::accounts::VerifyCollectionMint {
            authority: payer.pubkey(),
            metadata: nft_metadata,
            mint: nft_keypair.pubkey(),
            mint_authority,
            collection_mint: collection_keypair.pubkey(),
            collection_metadata,
            collection_master_edition,
            system_program: system_program::id(),
            sysvar_instruction: instructions_sysvar_id(),
            token_metadata_program: metadata_program_id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![verify_ix],
        &[&payer],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify the metadata still exists after verification
    let nft_meta = svm
        .get_account(&nft_metadata)
        .expect("NFT metadata should still exist after verification");
    assert!(!nft_meta.data.is_empty());
}
