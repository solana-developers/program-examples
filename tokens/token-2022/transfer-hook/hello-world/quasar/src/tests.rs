extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_transfer_hook_hello_world.so").unwrap();
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
fn test_initialize_mint_with_transfer_hook() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_2022_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;

    // 8-byte discriminator [0,0,0,0,0,0,0,1] + decimals = 2
    let data = vec![0, 0, 0, 0, 0, 0, 0, 1, 2];

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

#[test]
fn test_initialize_extra_account_meta_list() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    // Derive the ExtraAccountMetaList PDA
    let (meta_list_pda, _bump) = Pubkey::find_program_address(
        &[b"extra-account-metas", mint.as_ref()],
        &crate::ID.into(),
    );

    // InitializeExtraAccountMetaList discriminator
    let data = vec![43, 34, 13, 49, 167, 88, 235, 235];

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(meta_list_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data,
    };

    // mint doesn't need to exist for PDA derivation, just needs an address
    let result = svm.process_instruction(
        &instruction,
        &[signer(payer), empty(meta_list_pda), empty(mint)],
    );

    result.print_logs();
    assert!(
        result.is_ok(),
        "initialize_extra_account_meta_list failed: {:?}",
        result.raw_result
    );
    println!(
        "  INIT_EXTRA_ACCOUNT_METAS CU: {}",
        result.compute_units_consumed
    );
}

#[test]
fn test_transfer_hook() {
    let mut svm = setup();

    let source_token = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let destination_token = Pubkey::new_unique();
    let owner = Pubkey::new_unique();

    // Derive ExtraAccountMetaList PDA
    let (meta_list_pda, _bump) = Pubkey::find_program_address(
        &[b"extra-account-metas", mint.as_ref()],
        &crate::ID.into(),
    );

    // Execute discriminator + amount (1u64 LE)
    let mut data = vec![105, 37, 101, 197, 75, 251, 102, 26];
    data.extend_from_slice(&1u64.to_le_bytes());

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new_readonly(source_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint.into(), false),
            solana_instruction::AccountMeta::new_readonly(destination_token.into(), false),
            solana_instruction::AccountMeta::new_readonly(owner.into(), false),
            solana_instruction::AccountMeta::new_readonly(meta_list_pda.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            empty(source_token),
            empty(mint),
            empty(destination_token),
            signer(owner),
            empty(meta_list_pda),
        ],
    );

    result.print_logs();
    assert!(
        result.is_ok(),
        "transfer_hook failed: {:?}",
        result.raw_result
    );
    println!("  TRANSFER_HOOK CU: {}", result.compute_units_consumed);
}
