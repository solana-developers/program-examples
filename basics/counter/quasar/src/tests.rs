use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_counter_client::{InitializeCounterInstruction, IncrementInstruction};

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_counter.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 10_000_000_000)
}

fn empty(address: Pubkey) -> Account {
    Account {
        address,
        lamports: 0,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    }
}

#[test]
fn test_initialize_counter() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    // Derive the counter PDA from ["counter", payer].
    let (counter, _) = Pubkey::find_program_address(
        &[b"counter", payer.as_ref()],
        &Pubkey::from(crate::ID),
    );

    let instruction: Instruction = InitializeCounterInstruction {
        payer: Address::from(payer.to_bytes()),
        counter: Address::from(counter.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[signer(payer), empty(counter)],
    );

    result.assert_success();

    // Verify the counter account was created with count = 0.
    let counter_account = result.account(&counter).unwrap();
    // Data: 1 byte discriminator (1) + 8 bytes u64 (0)
    assert_eq!(counter_account.data.len(), 9);
    assert_eq!(counter_account.data[0], 1); // discriminator
    assert_eq!(&counter_account.data[1..], &[0u8; 8]); // count = 0
}

#[test]
fn test_increment() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    // Derive the counter PDA.
    let (counter, _) = Pubkey::find_program_address(
        &[b"counter", payer.as_ref()],
        &Pubkey::from(crate::ID),
    );

    // First, initialise the counter.
    let init_instruction: Instruction = InitializeCounterInstruction {
        payer: Address::from(payer.to_bytes()),
        counter: Address::from(counter.to_bytes()),
        system_program: Address::from(system_program.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &init_instruction,
        &[signer(payer), empty(counter)],
    );
    result.assert_success();

    // Grab updated accounts after init.
    let counter_after_init = result.account(&counter).unwrap().clone();

    // Increment the counter.
    let increment_instruction: Instruction = IncrementInstruction {
        counter: Address::from(counter.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &increment_instruction,
        &[counter_after_init],
    );
    result.assert_success();

    // Verify count = 1.
    let counter_account = result.account(&counter).unwrap();
    let count_bytes: [u8; 8] = counter_account.data[1..9].try_into().unwrap();
    let count = u64::from_le_bytes(count_bytes);
    assert_eq!(count, 1, "counter should be 1 after one increment");
}
