use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{
        create_associated_token_account, create_token_mint, create_wallet,
        get_token_account_balance, mint_tokens_to_token_account, send_transaction_from_instructions,
    },
    solana_signer::Signer,
};

fn token_program_id() -> Pubkey {
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        .parse()
        .unwrap()
}

fn derive_ata(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata_program: Pubkey = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        .parse()
        .unwrap();
    let (ata, _bump) = Pubkey::find_program_address(
        &[wallet.as_ref(), token_program_id().as_ref(), mint.as_ref()],
        &ata_program,
    );
    ata
}

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = external_delegate_token_master::id();
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/external_delegate_token_master.so");
    svm.add_program(program_id, program_bytes).unwrap();

    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

#[test]
fn test_initialize_user_account() {
    let (mut svm, program_id, authority) = setup();
    let user_account = Keypair::new();

    let init_ix = Instruction::new_with_bytes(
        program_id,
        &external_delegate_token_master::instruction::Initialize {}.data(),
        external_delegate_token_master::accounts::Initialize {
            user_account: user_account.pubkey(),
            authority: authority.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&authority, &user_account],
        &authority.pubkey(),
    )
    .unwrap();

    // Verify the account was created
    let account_data = svm
        .get_account(&user_account.pubkey())
        .expect("User account should exist");

    // Skip 8-byte discriminator
    let data = &account_data.data[8..];
    let stored_authority = Pubkey::try_from(&data[0..32]).unwrap();
    assert_eq!(stored_authority, authority.pubkey());

    // ethereum_address: [u8; 20] — should be all zeros
    let eth_addr = &data[32..52];
    assert_eq!(eth_addr, &[0u8; 20]);
}

#[test]
fn test_set_ethereum_address() {
    let (mut svm, program_id, authority) = setup();
    let user_account = Keypair::new();

    // Initialize
    let init_ix = Instruction::new_with_bytes(
        program_id,
        &external_delegate_token_master::instruction::Initialize {}.data(),
        external_delegate_token_master::accounts::Initialize {
            user_account: user_account.pubkey(),
            authority: authority.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&authority, &user_account],
        &authority.pubkey(),
    )
    .unwrap();

    // Set ethereum address
    let ethereum_address: [u8; 20] = [
        0x1C, 0x8c, 0xd0, 0xc3, 0x8F, 0x8D, 0xE3, 0x5d, 0x60, 0x56, 0xc7, 0xC7, 0xaB, 0xFa,
        0x7e, 0x65, 0xD2, 0x60, 0xE8, 0x16,
    ];

    let set_eth_ix = Instruction::new_with_bytes(
        program_id,
        &external_delegate_token_master::instruction::SetEthereumAddress {
            ethereum_address,
        }
        .data(),
        external_delegate_token_master::accounts::SetEthereumAddress {
            user_account: user_account.pubkey(),
            authority: authority.pubkey(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![set_eth_ix],
        &[&authority],
        &authority.pubkey(),
    )
    .unwrap();

    // Verify
    let account_data = svm
        .get_account(&user_account.pubkey())
        .expect("User account should exist");
    let data = &account_data.data[8..];
    let stored_eth_addr = &data[32..52];
    assert_eq!(stored_eth_addr, &ethereum_address);
}

#[test]
fn test_authority_transfer() {
    let (mut svm, program_id, authority) = setup();
    let user_account = Keypair::new();

    // Initialize user account
    let init_ix = Instruction::new_with_bytes(
        program_id,
        &external_delegate_token_master::instruction::Initialize {}.data(),
        external_delegate_token_master::accounts::Initialize {
            user_account: user_account.pubkey(),
            authority: authority.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&authority, &user_account],
        &authority.pubkey(),
    )
    .unwrap();

    // user_pda is derived from user_account key
    let (user_pda, _bump) =
        Pubkey::find_program_address(&[user_account.pubkey().as_ref()], &program_id);

    // Create mint and token accounts using Kite
    let mint_pubkey = create_token_mint(&mut svm, &authority, 6, None).unwrap();

    // Create ATA for the user_pda
    let user_pda_ata =
        create_associated_token_account(&mut svm, &user_pda, &mint_pubkey, &authority).unwrap();

    // Mint tokens to user_pda's ATA
    let mint_amount: u64 = 1_000_000_000;
    mint_tokens_to_token_account(&mut svm, &mint_pubkey, &user_pda_ata, mint_amount, &authority)
        .unwrap();

    // Create recipient ATA
    let recipient = Keypair::new();
    let recipient_ata =
        create_associated_token_account(&mut svm, &recipient.pubkey(), &mint_pubkey, &authority)
            .unwrap();

    // Perform authority transfer
    let transfer_amount: u64 = 500_000_000;
    let authority_transfer_ix = Instruction::new_with_bytes(
        program_id,
        &external_delegate_token_master::instruction::AuthorityTransfer {
            amount: transfer_amount,
        }
        .data(),
        external_delegate_token_master::accounts::AuthorityTransfer {
            user_account: user_account.pubkey(),
            authority: authority.pubkey(),
            user_token_account: user_pda_ata,
            recipient_token_account: recipient_ata,
            user_pda,
            token_program: token_program_id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![authority_transfer_ix],
        &[&authority],
        &authority.pubkey(),
    )
    .unwrap();

    // Verify recipient received tokens
    assert_eq!(
        get_token_account_balance(&svm, &recipient_ata).unwrap(),
        transfer_amount
    );

    // Verify user_pda's balance decreased
    assert_eq!(
        get_token_account_balance(&svm, &user_pda_ata).unwrap(),
        mint_amount - transfer_amount
    );
}
