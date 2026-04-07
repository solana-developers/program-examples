use {
    anchor_lang::{
        solana_program::{instruction::Instruction, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

/// PowerStatus account layout: 8-byte discriminator + 1-byte bool + 7 bytes padding.
/// Account space is 8 + 8 = 16 bytes, so read raw bytes to avoid "Not all bytes read" errors.
fn read_power_is_on(svm: &LiteSVM, pubkey: &anchor_lang::prelude::Pubkey) -> bool {
    let account = svm.get_account(pubkey).unwrap();
    account.data[8] != 0
}

#[test]
fn test_initialize_lever() {
    let program_id = lever::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/lever.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let power_keypair = Keypair::new();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &lever::instruction::Initialize {}.data(),
        lever::accounts::InitializeLever {
            power: power_keypair.pubkey(),
            user: payer.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![instruction],
        &[&payer, &power_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    assert!(
        !read_power_is_on(&svm, &power_keypair.pubkey()),
        "Power should be off after initialization"
    );
}

#[test]
fn test_switch_power() {
    let program_id = lever::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/lever.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let power_keypair = Keypair::new();

    // Initialize
    let init_ix = Instruction::new_with_bytes(
        program_id,
        &lever::instruction::Initialize {}.data(),
        lever::accounts::InitializeLever {
            power: power_keypair.pubkey(),
            user: payer.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&payer, &power_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Switch power on
    let switch_ix = Instruction::new_with_bytes(
        program_id,
        &lever::instruction::SwitchPower {
            name: "Alice".to_string(),
        }
        .data(),
        lever::accounts::SetPowerStatus {
            power: power_keypair.pubkey(),
        }
        .to_account_metas(None),
    );
    svm.expire_blockhash();
    send_transaction_from_instructions(&mut svm, vec![switch_ix], &[&payer], &payer.pubkey())
        .unwrap();

    assert!(
        read_power_is_on(&svm, &power_keypair.pubkey()),
        "Power should be on after first switch"
    );

    // Switch power off
    let switch_ix2 = Instruction::new_with_bytes(
        program_id,
        &lever::instruction::SwitchPower {
            name: "Bob".to_string(),
        }
        .data(),
        lever::accounts::SetPowerStatus {
            power: power_keypair.pubkey(),
        }
        .to_account_metas(None),
    );
    svm.expire_blockhash();
    send_transaction_from_instructions(&mut svm, vec![switch_ix2], &[&payer], &payer.pubkey())
        .unwrap();

    assert!(
        !read_power_is_on(&svm, &power_keypair.pubkey()),
        "Power should be off after second switch"
    );
}
