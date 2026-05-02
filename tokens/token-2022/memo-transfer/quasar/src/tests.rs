extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::Mint,
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_token_2022_memo_transfer.so").unwrap();
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

fn mint_account(address: Pubkey, authority: Pubkey) -> Account {
    quasar_svm::token::create_keyed_mint_account_with_program(
        &address,
        &Mint {
            mint_authority: Some(authority).into(),
            supply: 0,
            decimals: 2,
            is_initialized: true,
            freeze_authority: None.into(),
        },
        &quasar_svm::SPL_TOKEN_2022_PROGRAM_ID,
    )
}

#[test]
fn test_initialize() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let token_acc = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_2022_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(payer.into(), true),
            solana_instruction::AccountMeta::new(token_acc.into(), true),
            solana_instruction::AccountMeta::new_readonly(mint_addr.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data: vec![0u8],
    };

    let result = svm.process_instruction(
        &instruction,
        &[signer(payer), empty(token_acc), mint_account(mint_addr, payer)],
    );

    result.print_logs();
    assert!(result.is_ok(), "initialize failed: {:?}", result.raw_result);
    println!("  INITIALIZE CU: {}", result.compute_units_consumed);
}
