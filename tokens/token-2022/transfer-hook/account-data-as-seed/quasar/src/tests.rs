extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_transfer_hook_account_data_as_seed.so").unwrap();
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
fn test_initialize_and_transfer_hook() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let (meta_list_pda, _) = Pubkey::find_program_address(
        &[b"extra-account-metas", mint.as_ref()],
        &crate::ID.into(),
    );

    let (counter_pda, _) = Pubkey::find_program_address(
        &[b"counter", payer.as_ref()],
        &crate::ID.into(),
    );

    // Initialize
    let init_data = vec![43, 34, 13, 49, 167, 88, 235, 235];
    let init_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new(counter_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data: init_data,
    };

    let result = svm.process_instruction(
        &init_ix,
        &[signer(payer), empty(meta_list_pda), empty(mint), empty(counter_pda)],
    );
    result.print_logs();
    assert!(result.is_ok(), "init failed: {:?}", result.raw_result);
    println!("  INIT CU: {}", result.compute_units_consumed);

    // Transfer hook
    let source_token = Pubkey::new_unique();
    let destination_token = Pubkey::new_unique();
    let owner = Pubkey::new_unique();

    let mut hook_data = vec![105, 37, 101, 197, 75, 251, 102, 26];
    hook_data.extend_from_slice(&1u64.to_le_bytes());

    let hook_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(source_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new_readonly(destination_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(owner.into(), false),
            solana_instruction::AccountMeta::new_readonly(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new(counter_pda.into(), false),
        ],
        data: hook_data,
    };

    let result = svm.process_instruction(
        &hook_ix,
        &[empty(source_token), empty(destination_token), signer(owner)],
    );
    result.print_logs();
    assert!(result.is_ok(), "transfer_hook failed: {:?}", result.raw_result);
    println!("  TRANSFER_HOOK CU: {}", result.compute_units_consumed);

    let counter_account = svm.get_account(&counter_pda).expect("counter missing");
    let counter = u64::from_le_bytes(counter_account.data[8..16].try_into().unwrap());
    assert_eq!(counter, 1, "counter should be 1");
}
