extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::{Account as TokenAccount, AccountState, Mint},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_token_2022_basics.so").unwrap();
    QuasarSvm::new()
        .with_program(&crate::ID, &elf)
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 1_000_000_000)
}

fn mint_account(address: Pubkey, authority: Pubkey) -> Account {
    quasar_svm::token::create_keyed_mint_account_with_program(
        &address,
        &Mint {
            mint_authority: Some(authority).into(),
            supply: 0,
            decimals: 6,
            is_initialized: true,
            freeze_authority: None.into(),
        },
        &quasar_svm::SPL_TOKEN_2022_PROGRAM_ID,
    )
}

fn token_account(address: Pubkey, mint: Pubkey, owner: Pubkey, amount: u64) -> Account {
    quasar_svm::token::create_keyed_token_account_with_program(
        &address,
        &TokenAccount {
            mint,
            owner,
            amount,
            state: AccountState::Initialized,
            ..TokenAccount::default()
        },
        &quasar_svm::SPL_TOKEN_2022_PROGRAM_ID,
    )
}

#[test]
fn test_mint_token() {
    let mut svm = setup();

    let authority = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let receiver_addr = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_2022_PROGRAM_ID;

    let amount = 1_000_000u64;
    let mut data = vec![0u8]; // discriminator = 0
    data.extend_from_slice(&amount.to_le_bytes());

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(authority.into(), true),
            solana_instruction::AccountMeta::new(mint_addr.into(), false),
            solana_instruction::AccountMeta::new(receiver_addr.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(authority),
            mint_account(mint_addr, authority),
            token_account(receiver_addr, mint_addr, authority, 0),
        ],
    );

    result.print_logs();
    assert!(result.is_ok(), "mint_token failed: {:?}", result.raw_result);
    println!("  MINT TOKEN CU: {}", result.compute_units_consumed);
}

#[test]
fn test_transfer_token() {
    let mut svm = setup();

    let sender = Pubkey::new_unique();
    let from_addr = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let to_addr = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_2022_PROGRAM_ID;

    let amount = 500u64;
    let mut data = vec![1u8]; // discriminator = 1
    data.extend_from_slice(&amount.to_le_bytes());

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(sender.into(), true),
            solana_instruction::AccountMeta::new(from_addr.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint_addr.into(), false),
            solana_instruction::AccountMeta::new(to_addr.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(sender),
            token_account(from_addr, mint_addr, sender, 1_000),
            mint_account(mint_addr, sender),
            token_account(to_addr, mint_addr, sender, 0),
        ],
    );

    result.print_logs();
    assert!(result.is_ok(), "transfer_token failed: {:?}", result.raw_result);
    println!("  TRANSFER TOKEN CU: {}", result.compute_units_consumed);
}
