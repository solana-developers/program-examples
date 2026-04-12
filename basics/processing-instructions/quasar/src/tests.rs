use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_processing_instructions.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 10_000_000_000)
}

/// Build go_to_park instruction data.
/// Wire format: [disc=0] [ZC: height(u32)] [name: u32 prefix + bytes]
fn build_go_to_park(name: &str, height: u32) -> Vec<u8> {
    let mut data = vec![0u8]; // discriminator = 0

    // Fixed ZC: height
    data.extend_from_slice(&height.to_le_bytes());

    // Dynamic String: name
    data.extend_from_slice(&(name.len() as u32).to_le_bytes());
    data.extend_from_slice(name.as_bytes());

    data
}

#[test]
fn test_tall_enough() {
    let mut svm = setup();
    let user = Pubkey::new_unique();

    let ix = Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(
                Address::from(user.to_bytes()),
                true,
            ),
        ],
        data: build_go_to_park("Alice", 6),
    };

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("Welcome to the park!"), "should welcome");
    assert!(logs.contains("tall enough to ride"), "should say tall enough");
}

#[test]
fn test_not_tall_enough() {
    let mut svm = setup();
    let user = Pubkey::new_unique();

    let ix = Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(
                Address::from(user.to_bytes()),
                true,
            ),
        ],
        data: build_go_to_park("Bob", 3),
    };

    let result = svm.process_instruction(&ix, &[signer(user)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("Welcome to the park!"), "should welcome");
    assert!(logs.contains("NOT tall enough"), "should say not tall enough");
}
