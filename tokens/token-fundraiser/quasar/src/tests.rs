extern crate std;
use {
    alloc::vec,
    alloc::vec::Vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::{Account as TokenAccount, AccountState, Mint},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_token_fundraiser.so").unwrap();
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

fn mint(address: Pubkey, authority: Pubkey) -> Account {
    quasar_svm::token::create_keyed_mint_account(
        &address,
        &Mint {
            mint_authority: Some(authority).into(),
            supply: 1_000_000_000,
            decimals: 9,
            is_initialized: true,
            freeze_authority: None.into(),
        },
    )
}

fn token(address: Pubkey, mint: Pubkey, owner: Pubkey, amount: u64) -> Account {
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

/// Build Fundraiser account data.
/// Layout: [disc:1] [maker:32] [mint_to_raise:32] [amount_to_raise:8]
///         [current_amount:8] [time_started:8] [duration:2] [bump:1]
fn fundraiser_data(
    maker: Pubkey,
    mint_to_raise: Pubkey,
    amount_to_raise: u64,
    current_amount: u64,
    time_started: i64,
    duration: u16,
    bump: u8,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(92);
    data.push(1u8); // discriminator
    data.extend_from_slice(maker.as_ref());
    data.extend_from_slice(mint_to_raise.as_ref());
    data.extend_from_slice(&amount_to_raise.to_le_bytes());
    data.extend_from_slice(&current_amount.to_le_bytes());
    data.extend_from_slice(&time_started.to_le_bytes());
    data.extend_from_slice(&duration.to_le_bytes());
    data.push(bump);
    data
}

fn fundraiser_account(
    address: Pubkey,
    maker: Pubkey,
    mint_to_raise: Pubkey,
    amount_to_raise: u64,
    current_amount: u64,
    bump: u8,
) -> Account {
    Account {
        address,
        lamports: 2_000_000,
        data: fundraiser_data(maker, mint_to_raise, amount_to_raise, current_amount, 0, 30, bump),
        owner: crate::ID,
        executable: false,
    }
}

/// Build Contributor account data.
/// Layout: [disc:1=2] [amount:8]
fn contributor_data(amount: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(9);
    data.push(2u8); // discriminator
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

fn contributor_account(address: Pubkey, amount: u64) -> Account {
    Account {
        address,
        lamports: 1_000_000,
        data: contributor_data(amount),
        owner: crate::ID,
        executable: false,
    }
}

/// Build initialize instruction data.
/// Wire format: [disc: u8 = 0] [amount_to_raise: u64 LE] [duration: u16 LE]
fn build_init_data(amount_to_raise: u64, duration: u16) -> Vec<u8> {
    let mut data = vec![0u8];
    data.extend_from_slice(&amount_to_raise.to_le_bytes());
    data.extend_from_slice(&duration.to_le_bytes());
    data
}

/// Build contribute instruction data.
/// Wire format: [disc: u8 = 1] [amount: u64 LE]
fn build_contribute_data(amount: u64) -> Vec<u8> {
    let mut data = vec![1u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Build check_contributions instruction data.
/// Wire format: [disc: u8 = 2]
fn build_check_data() -> Vec<u8> {
    vec![2u8]
}

/// Build refund instruction data.
/// Wire format: [disc: u8 = 3]
fn build_refund_data() -> Vec<u8> {
    vec![3u8]
}

fn with_signers(mut ix: Instruction, indices: &[usize]) -> Instruction {
    for &i in indices {
        ix.accounts[i].is_signer = true;
    }
    ix
}

#[test]
fn test_initialize() {
    let mut svm = setup();

    let maker = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let vault = Pubkey::new_unique();
    let (fundraiser_pda, _) =
        Pubkey::find_program_address(&[b"fundraiser", maker.as_ref()], &crate::ID);
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;
    let rent = quasar_svm::solana_sdk_ids::sysvar::rent::ID;

    let data = build_init_data(10_000, 30);

    let instruction = with_signers(
        Instruction {
            program_id: crate::ID,
            accounts: vec![
                solana_instruction::AccountMeta::new(maker.into(), true),
                solana_instruction::AccountMeta::new_readonly(mint_addr.into(), false),
                solana_instruction::AccountMeta::new(fundraiser_pda.into(), false),
                solana_instruction::AccountMeta::new(vault.into(), false),
                solana_instruction::AccountMeta::new_readonly(rent.into(), false),
                solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
                solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
            ],
            data,
        },
        &[3], // vault as signer for create_account CPI
    );

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(maker),
            mint(mint_addr, maker),
            empty(fundraiser_pda),
            empty(vault),
        ],
    );

    assert!(result.is_ok(), "initialize failed: {:?}", result.raw_result);
    println!("  INITIALIZE CU: {}", result.compute_units_consumed);
}

#[test]
fn test_contribute() {
    let mut svm = setup();

    let contributor = Pubkey::new_unique();
    let maker = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let contributor_ta = Pubkey::new_unique();
    let vault_ta = Pubkey::new_unique();
    let contributor_acct = Pubkey::new_unique();
    let (fundraiser_pda, fundraiser_bump) =
        Pubkey::find_program_address(&[b"fundraiser", maker.as_ref()], &crate::ID);
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;

    let amount = 500u64;
    let data = build_contribute_data(amount);

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(contributor.into(), true),
            solana_instruction::AccountMeta::new(fundraiser_pda.into(), false),
            solana_instruction::AccountMeta::new(contributor_acct.into(), false),
            solana_instruction::AccountMeta::new(contributor_ta.into(), false),
            solana_instruction::AccountMeta::new(vault_ta.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(contributor),
            fundraiser_account(fundraiser_pda, maker, mint_addr, 10_000, 0, fundraiser_bump),
            contributor_account(contributor_acct, 0),
            token(contributor_ta, mint_addr, contributor, 100_000),
            token(vault_ta, mint_addr, fundraiser_pda, 0),
        ],
    );

    assert!(result.is_ok(), "contribute failed: {:?}", result.raw_result);
    println!("  CONTRIBUTE CU: {}", result.compute_units_consumed);
}

#[test]
fn test_check_contributions() {
    let mut svm = setup();

    let maker = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let vault_ta = Pubkey::new_unique();
    let maker_ta = Pubkey::new_unique();
    let (fundraiser_pda, fundraiser_bump) =
        Pubkey::find_program_address(&[b"fundraiser", maker.as_ref()], &crate::ID);
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;

    let data = build_check_data();

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(maker.into(), true),
            solana_instruction::AccountMeta::new(fundraiser_pda.into(), false),
            solana_instruction::AccountMeta::new(vault_ta.into(), false),
            solana_instruction::AccountMeta::new(maker_ta.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data,
    };

    // Target was 10_000, current is 10_000 — should succeed
    let result = svm.process_instruction(
        &instruction,
        &[
            signer(maker),
            fundraiser_account(fundraiser_pda, maker, mint_addr, 10_000, 10_000, fundraiser_bump),
            token(vault_ta, mint_addr, fundraiser_pda, 10_000),
            token(maker_ta, mint_addr, maker, 0),
        ],
    );

    assert!(result.is_ok(), "check_contributions failed: {:?}", result.raw_result);
    println!("  CHECK CONTRIBUTIONS CU: {}", result.compute_units_consumed);
}

#[test]
fn test_refund() {
    let mut svm = setup();

    let contributor = Pubkey::new_unique();
    let maker = Pubkey::new_unique();
    let mint_addr = Pubkey::new_unique();
    let contributor_ta = Pubkey::new_unique();
    let vault_ta = Pubkey::new_unique();
    let contributor_acct = Pubkey::new_unique();
    let (fundraiser_pda, fundraiser_bump) =
        Pubkey::find_program_address(&[b"fundraiser", maker.as_ref()], &crate::ID);
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;

    let refund_amount = 500u64;
    let data = build_refund_data();

    let instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![
            solana_instruction::AccountMeta::new(contributor.into(), true),
            solana_instruction::AccountMeta::new_readonly(maker.into(), false),
            solana_instruction::AccountMeta::new(fundraiser_pda.into(), false),
            solana_instruction::AccountMeta::new(contributor_acct.into(), false),
            solana_instruction::AccountMeta::new(contributor_ta.into(), false),
            solana_instruction::AccountMeta::new(vault_ta.into(), false),
            solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(contributor),
            signer(maker),
            fundraiser_account(fundraiser_pda, maker, mint_addr, 10_000, refund_amount, fundraiser_bump),
            contributor_account(contributor_acct, refund_amount),
            token(contributor_ta, mint_addr, contributor, 0),
            token(vault_ta, mint_addr, fundraiser_pda, refund_amount),
        ],
    );

    assert!(result.is_ok(), "refund failed: {:?}", result.raw_result);
    println!("  REFUND CU: {}", result.compute_units_consumed);
}
