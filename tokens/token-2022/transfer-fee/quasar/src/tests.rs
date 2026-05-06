extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_token_2022_transfer_fee.so").unwrap();
    QuasarSvm::new().with_program(&crate::ID, &elf)
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
fn test_initialize() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_2022_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;

    let basis_points: u16 = 100; // 1%
    let max_fee: u64 = 1_000_000;
    let mut data = vec![0u8]; // discriminator = 0
    data.extend_from_slice(&basis_points.to_le_bytes());
    data.extend_from_slice(&max_fee.to_le_bytes());

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(mint.into(), true),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[signer(payer), empty(mint)],
    );

    result.print_logs();
    assert!(result.is_ok(), "initialize failed: {:?}", result.raw_result);
    println!("  INITIALIZE CU: {}", result.compute_units_consumed);
}
