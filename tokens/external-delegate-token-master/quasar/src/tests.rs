extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_external_delegate_token_master.so").unwrap();
    QuasarSvm::new()
        .with_program(&crate::ID, &elf)
        .with_token_program()
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 5_000_000_000)
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

/// Build initialize instruction data.
/// Wire format: [disc=0]
fn build_initialize_data() -> Vec<u8> {
    vec![0u8]
}

#[test]
fn test_initialize() {
    let mut svm = setup();

    let authority = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let data = build_initialize_data();

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(user_account.into(), true),
            solana_instruction::AccountMeta::new(authority.into(), true),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[empty(user_account), signer(authority)],
    );

    assert!(
        result.is_ok(),
        "initialize failed: {:?}",
        result.raw_result
    );
    println!("  INITIALIZE CU: {}", result.compute_units_consumed);
}
