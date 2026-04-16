extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::{Account as TokenAccount, AccountState, Mint},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_token_2022_cpi_guard.so").unwrap();
    QuasarSvm::new().with_program(&crate::ID, &elf)
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 1_000_000_000)
}

fn mint_account(address: Pubkey, authority: Pubkey) -> Account {
    quasar_svm::token::create_keyed_mint_account_with_program(
        &address,
        &Mint {
            mint_authority: Some(authority).into(),
            supply: 1_000,
            decimals: 9,
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

/// Test CPI transfer_checked (without CPI guard — should succeed).
#[test]
fn test_cpi_transfer() {
    let mut svm = setup();

    let sender = Pubkey::new_unique();
    let sender_ta = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let recipient_ta = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_2022_PROGRAM_ID;

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(sender.into(), true),
            solana_instruction::AccountMeta::new(sender_ta.into(), false),
            solana_instruction::AccountMeta::new_readonly(mint_addr.into(), false),
            solana_instruction::AccountMeta::new(recipient_ta.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data: vec![0u8],
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(sender),
            token_account(sender_ta, mint_addr, sender, 100),
            mint_account(mint_addr, sender),
            token_account(recipient_ta, mint_addr, sender, 0),
        ],
    );

    result.print_logs();
    assert!(result.is_ok(), "cpi_transfer failed: {:?}", result.raw_result);
    println!("  CPI TRANSFER CU: {}", result.compute_units_consumed);
}
