use {
    anchor_lang::{
        solana_program::{instruction::Instruction, system_program},
        InstructionData, ToAccountMetas,
    },
    borsh::BorshDeserialize,
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

#[derive(BorshDeserialize)]
struct MessageAccount {
    _discriminator: [u8; 8],
    message: String,
}

fn fetch_message(svm: &LiteSVM, pubkey: &anchor_lang::prelude::Pubkey) -> String {
    let account = svm.get_account(pubkey).unwrap();
    let data = MessageAccount::try_from_slice(&account.data).unwrap();
    data.message
}

#[test]
fn test_initialize() {
    let program_id = anchor_realloc::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/anchor_realloc.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let message_keypair = Keypair::new();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &anchor_realloc::instruction::Initialize {
            input: "hello".to_string(),
        }
        .data(),
        anchor_realloc::accounts::Initialize {
            payer: payer.pubkey(),
            message_account: message_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![instruction],
        &[&payer, &message_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    let msg_text = fetch_message(&svm, &message_keypair.pubkey());
    assert_eq!(msg_text, "hello");

    // Verify account size: 8 (discriminator) + 4 (string length) + 5 ("hello")
    let account = svm.get_account(&message_keypair.pubkey()).unwrap();
    assert_eq!(account.data.len(), 8 + 4 + 5);
}

#[test]
fn test_update_grows() {
    let program_id = anchor_realloc::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/anchor_realloc.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let message_keypair = Keypair::new();

    // Initialize with "hello"
    let init_ix = Instruction::new_with_bytes(
        program_id,
        &anchor_realloc::instruction::Initialize {
            input: "hello".to_string(),
        }
        .data(),
        anchor_realloc::accounts::Initialize {
            payer: payer.pubkey(),
            message_account: message_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&payer, &message_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Update to "hello world" (grows the account)
    svm.expire_blockhash();
    let update_ix = Instruction::new_with_bytes(
        program_id,
        &anchor_realloc::instruction::Update {
            input: "hello world".to_string(),
        }
        .data(),
        anchor_realloc::accounts::Update {
            payer: payer.pubkey(),
            message_account: message_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![update_ix], &[&payer], &payer.pubkey())
        .unwrap();

    let msg_text = fetch_message(&svm, &message_keypair.pubkey());
    assert_eq!(msg_text, "hello world");

    let account = svm.get_account(&message_keypair.pubkey()).unwrap();
    assert_eq!(account.data.len(), 8 + 4 + 11);
}

#[test]
fn test_update_shrinks() {
    let program_id = anchor_realloc::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/anchor_realloc.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let message_keypair = Keypair::new();

    // Initialize with "hello world"
    let init_ix = Instruction::new_with_bytes(
        program_id,
        &anchor_realloc::instruction::Initialize {
            input: "hello world".to_string(),
        }
        .data(),
        anchor_realloc::accounts::Initialize {
            payer: payer.pubkey(),
            message_account: message_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&payer, &message_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Update to "hi" (shrinks the account)
    svm.expire_blockhash();
    let update_ix = Instruction::new_with_bytes(
        program_id,
        &anchor_realloc::instruction::Update {
            input: "hi".to_string(),
        }
        .data(),
        anchor_realloc::accounts::Update {
            payer: payer.pubkey(),
            message_account: message_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![update_ix], &[&payer], &payer.pubkey())
        .unwrap();

    let msg_text = fetch_message(&svm, &message_keypair.pubkey());
    assert_eq!(msg_text, "hi");

    let account = svm.get_account(&message_keypair.pubkey()).unwrap();
    assert_eq!(account.data.len(), 8 + 4 + 2);
}
