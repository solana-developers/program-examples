extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::{Account as TokenAccount, AccountState, Mint},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_spl_token_minter.so").unwrap();
    QuasarSvm::new()
        .with_program(&crate::ID, &elf)
        .with_token_program()
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 5_000_000_000)
}

fn mint(address: Pubkey, authority: Pubkey) -> Account {
    quasar_svm::token::create_keyed_mint_account(
        &address,
        &Mint {
            mint_authority: Some(authority).into(),
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: Some(authority).into(),
        },
    )
}

fn token_account(address: Pubkey, mint_address: Pubkey, owner: Pubkey, amount: u64) -> Account {
    quasar_svm::token::create_keyed_token_account(
        &address,
        &TokenAccount {
            mint: mint_address,
            owner,
            amount,
            state: AccountState::Initialized,
            ..TokenAccount::default()
        },
    )
}

/// Build mint_token instruction data.
/// Wire format: [disc=1] [amount: u64 LE]
fn build_mint_token_data(amount: u64) -> Vec<u8> {
    let mut data = vec![1u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

// Note: create_token test requires the Metaplex Token Metadata program
// deployed in the SVM. The quasar-svm harness does not currently ship it,
// so we test mint_token (pure SPL Token CPI) only.

#[test]
fn test_mint_token() {
    let mut svm = setup();

    let authority = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();
    let mint_address = Pubkey::new_unique();
    let token_addr = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;

    let amount = 100u64;
    let data = build_mint_token_data(amount);

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(authority.into(), true),
            solana_instruction::AccountMeta::new_readonly(recipient.into(), false),
            solana_instruction::AccountMeta::new(mint_address.into(), false),
            solana_instruction::AccountMeta::new(token_addr.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
            solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(authority),
            signer(recipient),
            mint(mint_address, authority),
            token_account(token_addr, mint_address, recipient, 0),
        ],
    );

    assert!(
        result.is_ok(),
        "mint_token failed: {:?}",
        result.raw_result
    );
    println!("  MINT TOKEN CU: {}", result.compute_units_consumed);
}
