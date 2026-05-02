extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_transfer_hook_switch.so").unwrap();
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
fn test_transfer_switch_flow() {
    let mut svm = setup();

    let admin = Pubkey::new_unique();
    let new_admin = Pubkey::new_unique();
    let wallet = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let (admin_config_pda, _) =
        Pubkey::find_program_address(&[b"admin-config"], &crate::ID.into());
    let (meta_list_pda, _) = Pubkey::find_program_address(
        &[b"extra-account-metas", mint.as_ref()],
        &crate::ID.into(),
    );
    let (wallet_switch_pda, _) =
        Pubkey::find_program_address(&[wallet.as_ref()], &crate::ID.into());

    // 1. Configure admin
    let config_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(admin.into(), true),
            solana_instruction::AccountMeta::new_readonly(new_admin.into(), false),
            solana_instruction::AccountMeta::new(admin_config_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data: vec![0, 0, 0, 0, 0, 0, 0, 1],
    };
    let result = svm.process_instruction(&config_ix, &[signer(admin), empty(admin_config_pda)]);
    result.print_logs();
    assert!(result.is_ok(), "configure_admin failed: {:?}", result.raw_result);
    println!("  CONFIGURE_ADMIN CU: {}", result.compute_units_consumed);

    // 2. Initialize extra account metas
    let init_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(admin.into(), true),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data: vec![43, 34, 13, 49, 167, 88, 235, 235],
    };
    let result = svm.process_instruction(&init_ix, &[signer(admin), empty(mint), empty(meta_list_pda)]);
    result.print_logs();
    assert!(result.is_ok(), "init_metas failed: {:?}", result.raw_result);

    // 3. Turn switch ON for wallet (new_admin is now the admin in the config)
    // discriminator [0,0,0,0,0,0,0,3] + on=1 (u8)
    let switch_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(new_admin.into(), true),
            solana_instruction::AccountMeta::new_readonly(wallet.into(), false),
            solana_instruction::AccountMeta::new_readonly(admin_config_pda.into(), false),
            solana_instruction::AccountMeta::new(wallet_switch_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data: vec![0, 0, 0, 0, 0, 0, 0, 3, 1],
    };
    let result = svm.process_instruction(
        &switch_ix,
        &[signer(new_admin), empty(wallet), empty(wallet_switch_pda)],
    );
    result.print_logs();
    assert!(result.is_ok(), "switch on failed: {:?}", result.raw_result);
    println!("  SWITCH ON CU: {}", result.compute_units_consumed);

    // 4. Transfer hook with switch ON — should succeed
    let source_token = Pubkey::new_unique();
    let dest_token = Pubkey::new_unique();

    let mut hook_data = vec![105, 37, 101, 197, 75, 251, 102, 26];
    hook_data.extend_from_slice(&100u64.to_le_bytes());

    let hook_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(source_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new_readonly(dest_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(wallet.into(), false),
            solana_instruction::AccountMeta::new_readonly(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(wallet_switch_pda.into(), false),
        ],
        data: hook_data.clone(),
    };
    let result = svm.process_instruction(
        &hook_ix,
        &[empty(source_token), empty(dest_token), signer(wallet)],
    );
    result.print_logs();
    assert!(result.is_ok(), "transfer_hook (switch on) failed: {:?}", result.raw_result);
    println!("  TRANSFER_HOOK (on) CU: {}", result.compute_units_consumed);

    // 5. Turn switch OFF (new_admin is the admin)
    let switch_off_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(new_admin.into(), true),
            solana_instruction::AccountMeta::new_readonly(wallet.into(), false),
            solana_instruction::AccountMeta::new_readonly(admin_config_pda.into(), false),
            solana_instruction::AccountMeta::new(wallet_switch_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data: vec![0, 0, 0, 0, 0, 0, 0, 3, 0],
    };
    let result = svm.process_instruction(
        &switch_off_ix,
        &[signer(new_admin), empty(wallet)],
    );
    result.print_logs();
    assert!(result.is_ok(), "switch off failed: {:?}", result.raw_result);

    // 6. Transfer hook with switch OFF — should fail
    let hook_ix2 = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(source_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new_readonly(dest_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(wallet.into(), false),
            solana_instruction::AccountMeta::new_readonly(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(wallet_switch_pda.into(), false),
        ],
        data: hook_data,
    };
    let result = svm.process_instruction(
        &hook_ix2,
        &[empty(source_token), empty(dest_token), signer(wallet)],
    );
    result.print_logs();
    assert!(result.is_err(), "transfer_hook should fail with switch off");
    println!("  TRANSFER_HOOK (off) correctly rejected");
}
