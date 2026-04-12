extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_transfer_hook_whitelist.so").unwrap();
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
fn test_whitelist_flow() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let (meta_list_pda, _) = Pubkey::find_program_address(
        &[b"extra-account-metas", mint.as_ref()],
        &crate::ID.into(),
    );
    let (white_list_pda, _) =
        Pubkey::find_program_address(&[b"white_list"], &crate::ID.into());

    // 1. Initialize
    let init_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new(white_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data: vec![43, 34, 13, 49, 167, 88, 235, 235],
    };

    let result = svm.process_instruction(
        &init_ix,
        &[signer(payer), empty(meta_list_pda), empty(mint), empty(white_list_pda)],
    );
    result.print_logs();
    assert!(result.is_ok(), "init failed: {:?}", result.raw_result);
    println!("  INIT CU: {}", result.compute_units_consumed);

    // 2. Add destination to whitelist
    let destination_token = Pubkey::new_unique();
    let add_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(payer.into(), true),
            solana_instruction::AccountMeta::new_readonly(destination_token.into(), false),
            solana_instruction::AccountMeta::new(white_list_pda.into(), false),
        ],
        data: vec![0, 0, 0, 0, 0, 0, 0, 2],
    };

    let result = svm.process_instruction(
        &add_ix,
        &[signer(payer), empty(destination_token)],
    );
    result.print_logs();
    assert!(result.is_ok(), "add_to_whitelist failed: {:?}", result.raw_result);

    // 3. Transfer hook with whitelisted destination — should succeed
    let source_token = Pubkey::new_unique();
    let owner = Pubkey::new_unique();

    let mut hook_data = vec![105, 37, 101, 197, 75, 251, 102, 26];
    hook_data.extend_from_slice(&100u64.to_le_bytes());

    let hook_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(source_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new_readonly(destination_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(owner.into(), false),
            solana_instruction::AccountMeta::new_readonly(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(white_list_pda.into(), false),
        ],
        data: hook_data,
    };

    let result = svm.process_instruction(
        &hook_ix,
        &[empty(source_token), empty(destination_token), signer(owner)],
    );
    result.print_logs();
    assert!(result.is_ok(), "transfer_hook (whitelisted) failed: {:?}", result.raw_result);
    println!("  TRANSFER_HOOK (allowed) CU: {}", result.compute_units_consumed);

    // 4. Transfer hook with non-whitelisted destination — should fail
    let bad_dest = Pubkey::new_unique();
    let mut hook_data2 = vec![105, 37, 101, 197, 75, 251, 102, 26];
    hook_data2.extend_from_slice(&100u64.to_le_bytes());

    let bad_hook_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(source_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new_readonly(bad_dest.into(), false),
            solana_instruction::AccountMeta::new_readonly(owner.into(), false),
            solana_instruction::AccountMeta::new_readonly(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(white_list_pda.into(), false),
        ],
        data: hook_data2,
    };

    let result = svm.process_instruction(
        &bad_hook_ix,
        &[empty(source_token), empty(bad_dest), signer(owner)],
    );
    result.print_logs();
    assert!(result.is_err(), "transfer_hook should fail for non-whitelisted destination");
    println!("  TRANSFER_HOOK (blocked) correctly rejected");
}
