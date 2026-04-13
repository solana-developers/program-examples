use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, get_token_account_balance, send_transaction_from_instructions},
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
    let program_id = transfer_tokens::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/transfer_tokens.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let metadata_bytes = include_bytes!("../../../tests/fixtures/mpl_token_metadata.so");
    svm.add_program(metadata_program_id(), metadata_bytes)
        .unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_create_mint_and_transfer() {
    let (mut svm, program_id, payer) = setup();
    let mint_keypair = Keypair::new();
    let metadata_account = derive_metadata_pda(&mint_keypair.pubkey());

    // 1. Create token (with metadata)
    let create_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_tokens::instruction::CreateToken {
            token_title: "Solana Gold".to_string(),
            token_symbol: "GOLDSOL".to_string(),
            token_uri: "https://example.com/token.json".to_string(),
        }
        .data(),
        transfer_tokens::accounts::CreateToken {
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
    let mint_account = svm
        .get_account(&mint_keypair.pubkey())
        .expect("Mint should exist");
    assert!(!mint_account.data.is_empty());

    // 2. Mint tokens (100 tokens to payer's ATA)
    svm.expire_blockhash();
    let sender_ata = derive_ata(&payer.pubkey(), &mint_keypair.pubkey());

    let mint_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_tokens::instruction::MintToken { amount: 100 }.data(),
        transfer_tokens::accounts::MintToken {
            mint_authority: payer.pubkey(),
            recipient: payer.pubkey(),
            mint_account: mint_keypair.pubkey(),
            associated_token_account: sender_ata,
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

    // Verify tokens minted — 100 * 10^9 = 100_000_000_000 (9 decimals)
    assert_eq!(
        get_token_account_balance(&svm, &sender_ata).unwrap(),
        100_000_000_000
    );

    // 3. Transfer tokens (50 tokens to recipient)
    svm.expire_blockhash();
    let recipient = Keypair::new();
    let recipient_ata = derive_ata(&recipient.pubkey(), &mint_keypair.pubkey());

    let transfer_ix = Instruction::new_with_bytes(
        program_id,
        &transfer_tokens::instruction::TransferTokens { amount: 50 }.data(),
        transfer_tokens::accounts::TransferTokens {
            sender: payer.pubkey(),
            recipient: recipient.pubkey(),
            mint_account: mint_keypair.pubkey(),
            sender_token_account: sender_ata,
            recipient_token_account: recipient_ata,
            token_program: token_program_id(),
            associated_token_program: associated_token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![transfer_ix],
        &[&payer],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify: sender 50 tokens, recipient 50 tokens (at 9 decimals)
    assert_eq!(
        get_token_account_balance(&svm, &sender_ata).unwrap(),
        50_000_000_000
    );
    assert_eq!(
        get_token_account_balance(&svm, &recipient_ata).unwrap(),
        50_000_000_000
    );
}
