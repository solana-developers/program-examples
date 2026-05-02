extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::{Account as TokenAccount, AccountState, Mint},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_pda_mint_authority.so").unwrap();
    QuasarSvm::new()
        .with_program(&crate::ID, &elf)
        .with_token_program()
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 1_000_000_000)
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

fn mint_account(address: Pubkey, authority: Pubkey) -> Account {
    quasar_svm::token::create_keyed_mint_account(
        &address,
        &Mint {
            mint_authority: Some(authority).into(),
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: None.into(),
        },
    )
}

fn token_account(address: Pubkey, mint: Pubkey, owner: Pubkey, amount: u64) -> Account {
    quasar_svm::token::create_keyed_token_account(
        &address,
        &TokenAccount {
            mint,
            owner,
            amount,
            state: AccountState::Initialized,
            ..TokenAccount::default()
        },
    )
}

/// Build create_mint instruction data.
/// Wire format: [discriminator: u8 = 0] [decimals: u8]
fn build_create_mint_data(decimals: u8) -> Vec<u8> {
    vec![0u8, decimals]
}

/// Build mint_tokens instruction data.
/// Wire format: [discriminator: u8 = 1] [amount: u64 LE]
fn build_mint_tokens_data(amount: u64) -> Vec<u8> {
    let mut data = vec![1u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

#[test]
fn test_create_mint() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let (mint_pda, _) = Pubkey::find_program_address(&[b"mint"], &crate::ID);
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;
    let rent = quasar_svm::solana_sdk_ids::sysvar::rent::ID;

    let data = build_create_mint_data(9);

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(mint_pda.into(), false),
            solana_instruction::AccountMeta::new_readonly(rent.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(&instruction, &[signer(payer), empty(mint_pda)]);
    assert!(result.is_ok(), "create_mint failed: {:?}", result.raw_result);
    println!("  CREATE MINT CU: {}", result.compute_units_consumed);
}

#[test]
fn test_mint_with_pda_authority() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let (mint_pda, _) = Pubkey::find_program_address(&[b"mint"], &crate::ID);
    let token_addr = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;

    let amount = 1_000_000_000u64;
    let data = build_mint_tokens_data(amount);

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(mint_pda.into(), false),
            solana_instruction::AccountMeta::new(token_addr.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(payer),
            // The mint authority is the mint_pda itself
            mint_account(mint_pda, mint_pda),
            token_account(token_addr, mint_pda, payer, 0),
        ],
    );

    assert!(result.is_ok(), "mint_tokens failed: {:?}", result.raw_result);
    println!("  MINT WITH PDA CU: {}", result.compute_units_consumed);
}
