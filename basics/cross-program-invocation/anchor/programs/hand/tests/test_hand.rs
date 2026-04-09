use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            system_program,
        },
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

/// PowerStatus account layout: 8-byte discriminator + 1-byte bool + 7 bytes padding.
/// Account space is 8 + 8 = 16 bytes, so read the raw bytes instead of using BorshDeserialize
/// to avoid "Not all bytes read" errors from the padding.
fn read_power_is_on(svm: &LiteSVM, pubkey: &anchor_lang::prelude::Pubkey) -> bool {
    let account = svm.get_account(pubkey).unwrap();
    // Skip 8-byte discriminator, read 1 byte for bool
    account.data[8] != 0
}

/// Build the lever program's `initialize` instruction manually.
/// Discriminator from IDL: [175, 175, 109, 31, 13, 152, 155, 237]
fn build_lever_initialize_ix(
    lever_program_id: anchor_lang::prelude::Pubkey,
    power: anchor_lang::prelude::Pubkey,
    user: anchor_lang::prelude::Pubkey,
) -> Instruction {
    let discriminator: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    Instruction {
        program_id: lever_program_id,
        accounts: vec![
            AccountMeta::new(power, true),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: discriminator.to_vec(),
    }
}

#[test]
fn test_pull_lever_cpi() {
    let hand_program_id = hand::id();
    // The lever program ID from declare_program!(lever) inside hand crate
    let lever_program_id = hand::lever::ID;

    let mut svm = LiteSVM::new();

    // Load both programs
    let hand_bytes = include_bytes!("../../../target/deploy/hand.so");
    // Use std::fs::read() instead of include_bytes!() for the lever program because
    // include_bytes!() runs at compile time, and during `anchor build` the IDL generation
    // step compiles tests before the .so files exist. Since this is a cross-program
    // dependency (not our own program), lever.so may not be built yet at compile time.
    let lever_bytes = std::fs::read("target/deploy/lever.so").expect("lever.so not found — run `anchor build` first");
    svm.add_program(hand_program_id, hand_bytes).unwrap();
    svm.add_program(lever_program_id, lever_bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let power_keypair = Keypair::new();

    // Initialize the lever directly (manually constructed instruction)
    let init_ix =
        build_lever_initialize_ix(lever_program_id, power_keypair.pubkey(), payer.pubkey());
    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&payer, &power_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Verify initial state is off
    assert!(
        !read_power_is_on(&svm, &power_keypair.pubkey()),
        "Power should be off after initialization"
    );

    // Pull the lever via the hand program (CPI into lever)
    svm.expire_blockhash();
    let pull_ix = Instruction::new_with_bytes(
        hand_program_id,
        &hand::instruction::PullLever {
            name: "Jacob".to_string(),
        }
        .data(),
        hand::accounts::PullLever {
            power: power_keypair.pubkey(),
            lever_program: lever_program_id,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![pull_ix], &[&payer], &payer.pubkey())
        .unwrap();

    // Verify power is now on
    assert!(
        read_power_is_on(&svm, &power_keypair.pubkey()),
        "Power should be on after pulling lever"
    );

    // Pull it again
    svm.expire_blockhash();
    let pull_ix2 = Instruction::new_with_bytes(
        hand_program_id,
        &hand::instruction::PullLever {
            name: "sol-warrior".to_string(),
        }
        .data(),
        hand::accounts::PullLever {
            power: power_keypair.pubkey(),
            lever_program: lever_program_id,
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![pull_ix2], &[&payer], &payer.pubkey())
        .unwrap();

    // Verify power is now off again
    assert!(
        !read_power_is_on(&svm, &power_keypair.pubkey()),
        "Power should be off after pulling lever again"
    );
}
