extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_token_2022_mint_close_authority.so").unwrap();
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
fn test_initialize_and_close() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_2022_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;

    // Initialize
    let init_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(mint.into(), true),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data: vec![0u8],
    };

    let result = svm.process_instruction(
        &init_ix,
        &[signer(payer), empty(mint)],
    );

    result.print_logs();
    assert!(result.is_ok(), "initialize failed: {:?}", result.raw_result);
    println!("  INITIALIZE CU: {}", result.compute_units_consumed);

    // Close
    let close_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(mint.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data: vec![1u8],
    };

    let close_result = svm.process_instruction_chain(&[close_ix.clone()], &result.accounts);

    close_result.print_logs();
    assert!(close_result.is_ok(), "close failed: {:?}", close_result.raw_result);
    println!("  CLOSE CU: {}", close_result.compute_units_consumed);
}
