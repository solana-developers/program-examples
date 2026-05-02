extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::{Account as TokenAccount, AccountState, Mint},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_transfer_tokens.so").unwrap();
    QuasarSvm::new()
        .with_program(&crate::ID, &elf)
        .with_token_program()
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 1_000_000_000)
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

/// Build mint_tokens instruction data.
/// Wire format: [discriminator: u8 = 0] [amount: u64 LE]
fn build_mint_data(amount: u64) -> Vec<u8> {
    let mut data = vec![0u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Build transfer_tokens instruction data.
/// Wire format: [discriminator: u8 = 1] [amount: u64 LE]
fn build_transfer_data(amount: u64) -> Vec<u8> {
    let mut data = vec![1u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

#[test]
fn test_mint_tokens() {
    let mut svm = setup();

    let authority = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let recipient_ta = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;

    let amount = 1_000_000_000u64;
    let data = build_mint_data(amount);

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(authority.into(), true),
            solana_instruction::AccountMeta::new(mint_addr.into(), false),
            solana_instruction::AccountMeta::new(recipient_ta.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(authority),
            mint_account(mint_addr, authority),
            token_account(recipient_ta, mint_addr, authority, 0),
        ],
    );

    assert!(result.is_ok(), "mint_tokens failed: {:?}", result.raw_result);
    println!("  MINT TOKENS CU: {}", result.compute_units_consumed);
}

#[test]
fn test_transfer_tokens() {
    let mut svm = setup();

    let sender = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let sender_ta = Pubkey::new_unique();
    let recipient_ta = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;

    let amount = 500u64;
    let data = build_transfer_data(amount);

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(sender.into(), true),
            solana_instruction::AccountMeta::new(sender_ta.into(), false),
            solana_instruction::AccountMeta::new(recipient_ta.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(sender),
            token_account(sender_ta, mint_addr, sender, 10_000),
            token_account(recipient_ta, mint_addr, recipient, 0),
        ],
    );

    assert!(result.is_ok(), "transfer_tokens failed: {:?}", result.raw_result);
    println!("  TRANSFER TOKENS CU: {}", result.compute_units_consumed);
}
