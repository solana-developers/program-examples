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

/// Minimal deserialization of the Counter account (8-byte discriminator + u64).
#[derive(BorshDeserialize)]
struct CounterAccount {
    _discriminator: [u8; 8],
    count: u64,
}

fn setup() -> (LiteSVM, anchor_lang::prelude::Pubkey, Keypair) {
    let program_id = counter_anchor::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/counter_anchor.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

fn fetch_counter(svm: &LiteSVM, counter_pubkey: &anchor_lang::prelude::Pubkey) -> u64 {
    let account = svm.get_account(counter_pubkey).unwrap();
    let counter = CounterAccount::try_from_slice(&account.data).unwrap();
    counter.count
}

#[test]
fn test_initialize_counter() {
    let (mut svm, _program_id, payer) = setup();
    let counter_keypair = Keypair::new();

    let instruction = Instruction::new_with_bytes(
        counter_anchor::id(),
        &counter_anchor::instruction::InitializeCounter {}.data(),
        counter_anchor::accounts::InitializeCounter {
            payer: payer.pubkey(),
            counter: counter_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![instruction],
        &[&payer, &counter_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    let count = fetch_counter(&svm, &counter_keypair.pubkey());
    assert_eq!(count, 0, "Expected initialized count to be 0");
}

#[test]
fn test_increment_counter() {
    let (mut svm, _program_id, payer) = setup();
    let counter_keypair = Keypair::new();

    // Initialize
    let init_ix = Instruction::new_with_bytes(
        counter_anchor::id(),
        &counter_anchor::instruction::InitializeCounter {}.data(),
        counter_anchor::accounts::InitializeCounter {
            payer: payer.pubkey(),
            counter: counter_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&payer, &counter_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Increment
    let inc_ix = Instruction::new_with_bytes(
        counter_anchor::id(),
        &counter_anchor::instruction::Increment {}.data(),
        counter_anchor::accounts::Increment {
            counter: counter_keypair.pubkey(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![inc_ix], &[&payer], &payer.pubkey()).unwrap();

    let count = fetch_counter(&svm, &counter_keypair.pubkey());
    assert_eq!(count, 1, "Expected count to be 1");
}

#[test]
fn test_increment_counter_again() {
    let (mut svm, _program_id, payer) = setup();
    let counter_keypair = Keypair::new();

    // Initialize
    let init_ix = Instruction::new_with_bytes(
        counter_anchor::id(),
        &counter_anchor::instruction::InitializeCounter {}.data(),
        counter_anchor::accounts::InitializeCounter {
            payer: payer.pubkey(),
            counter: counter_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(
        &mut svm,
        vec![init_ix],
        &[&payer, &counter_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Increment twice
    for _ in 0..2 {
        let inc_ix = Instruction::new_with_bytes(
            counter_anchor::id(),
            &counter_anchor::instruction::Increment {}.data(),
            counter_anchor::accounts::Increment {
                counter: counter_keypair.pubkey(),
            }
            .to_account_metas(None),
        );
        send_transaction_from_instructions(&mut svm, vec![inc_ix], &[&payer], &payer.pubkey())
            .unwrap();
        svm.expire_blockhash();
    }

    let count = fetch_counter(&svm, &counter_keypair.pubkey());
    assert_eq!(count, 2, "Expected count to be 2");
}
