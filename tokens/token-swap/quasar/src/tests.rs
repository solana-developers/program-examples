extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_token_swap.so").unwrap();
    QuasarSvm::new()
        .with_program(&crate::ID, &elf)
        .with_token_program()
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

fn build_create_amm_data(id: &Pubkey, fee: u16) -> Vec<u8> {
    let mut data = vec![0u8]; // discriminator
    data.extend_from_slice(id.as_ref());
    data.extend_from_slice(&fee.to_le_bytes());
    data
}

#[test]
fn test_create_amm() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let admin = Pubkey::new_unique();
    let amm_id = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    // Derive the AMM PDA
    let (amm_pda, _) = Pubkey::find_program_address(&[b"amm"], &crate::ID.into());

    let data = build_create_amm_data(&amm_id, 30);

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(amm_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(admin.into(), false),
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[empty(amm_pda), signer(admin), signer(payer)],
    );

    assert!(
        result.is_ok(),
        "create_amm failed: {:?}",
        result.raw_result
    );
    println!("  CREATE AMM CU: {}", result.compute_units_consumed);
}

#[test]
fn test_create_amm_invalid_fee() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let admin = Pubkey::new_unique();
    let amm_id = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let (amm_pda, _) = Pubkey::find_program_address(&[b"amm"], &crate::ID.into());

    // Fee >= 10000 should fail.
    let data = build_create_amm_data(&amm_id, 10000);

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(amm_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(admin.into(), false),
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[empty(amm_pda), signer(admin), signer(payer)],
    );

    assert!(
        !result.is_ok(),
        "create_amm should have failed with invalid fee"
    );
    println!("  CREATE AMM (invalid fee) correctly rejected");
}
