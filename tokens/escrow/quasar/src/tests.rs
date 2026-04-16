extern crate std;
use {
    alloc::vec,
    alloc::vec::Vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::{Account as TokenAccount, AccountState, Mint},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_escrow.so").unwrap();
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

/// Build escrow account data manually.
/// Layout (from #[account] codegen):
///   [disc: 1 byte = 1]
///   [maker: 32 bytes (Address)]
///   [mint_a: 32 bytes]
///   [mint_b: 32 bytes]
///   [maker_ta_b: 32 bytes]
///   [receive: 8 bytes (PodU64 LE)]
///   [bump: 1 byte]
/// Total: 138 bytes
fn escrow_data(
    maker: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    maker_ta_b: Pubkey,
    receive: u64,
    bump: u8,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(138);
    data.push(1u8); // discriminator
    data.extend_from_slice(maker.as_ref());
    data.extend_from_slice(mint_a.as_ref());
    data.extend_from_slice(mint_b.as_ref());
    data.extend_from_slice(maker_ta_b.as_ref());
    data.extend_from_slice(&receive.to_le_bytes());
    data.push(bump);
    data
}

fn escrow_account(
    address: Pubkey,
    maker: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    maker_ta_b: Pubkey,
    receive: u64,
    bump: u8,
) -> Account {
    Account {
        address,
        lamports: 2_000_000,
        data: escrow_data(maker, mint_a, mint_b, maker_ta_b, receive, bump),
        owner: crate::ID,
        executable: false,
    }
}

/// Mark specific account indices as signers on an instruction.
fn with_signers(mut ix: Instruction, indices: &[usize]) -> Instruction {
    for &i in indices {
        ix.accounts[i].is_signer = true;
    }
    ix
}

/// Build make instruction data.
/// Wire format: [discriminator: u8 = 0] [deposit: u64 LE] [receive: u64 LE]
fn build_make_data(deposit: u64, receive: u64) -> Vec<u8> {
    let mut data = vec![0u8];
    data.extend_from_slice(&deposit.to_le_bytes());
    data.extend_from_slice(&receive.to_le_bytes());
    data
}

/// Build take instruction data.
/// Wire format: [discriminator: u8 = 1]
fn build_take_data() -> Vec<u8> {
    vec![1u8]
}

/// Build refund instruction data.
/// Wire format: [discriminator: u8 = 2]
fn build_refund_data() -> Vec<u8> {
    vec![2u8]
}

#[test]
fn test_make() {
    let mut svm = setup();

    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;
    let maker = Pubkey::new_unique();
    let mint_a = Pubkey::new_unique();
    let mint_b = Pubkey::new_unique();
    let maker_ta_a = Pubkey::new_unique();
    let maker_ta_b = Pubkey::new_unique();
    let vault_ta_a = Pubkey::new_unique();
    let (escrow, escrow_bump) =
        Pubkey::find_program_address(&[b"escrow", maker.as_ref()], &crate::ID);
    let rent = quasar_svm::solana_sdk_ids::sysvar::rent::ID;

    let data = build_make_data(1337, 1337);

    let instruction = with_signers(
        Instruction {
            program_id: crate::ID,
            accounts: vec![
                solana_instruction::AccountMeta::new(maker.into(), true),
                solana_instruction::AccountMeta::new(escrow.into(), false),
                solana_instruction::AccountMeta::new_readonly(mint_a.into(), false),
                solana_instruction::AccountMeta::new_readonly(mint_b.into(), false),
                solana_instruction::AccountMeta::new(maker_ta_a.into(), false),
                solana_instruction::AccountMeta::new(maker_ta_b.into(), false),
                solana_instruction::AccountMeta::new(vault_ta_a.into(), false),
                solana_instruction::AccountMeta::new_readonly(rent.into(), false),
                solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
                solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
            ],
            data,
        },
        &[5, 6], // maker_ta_b, vault_ta_a as signers for create_account CPI
    );

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(maker),
            empty(escrow),
            mint(mint_a, maker),
            mint(mint_b, maker),
            token(maker_ta_a, mint_a, maker, 1_000_000),
            empty(maker_ta_b),
            empty(vault_ta_a),
        ],
    );

    assert!(result.is_ok(), "make failed: {:?}", result.raw_result);

    // Verify escrow state
    let escrow_data = &result.account(&escrow).unwrap().data;
    assert_eq!(escrow_data[0], 1, "discriminator");
    assert_eq!(&escrow_data[1..33], maker.as_ref(), "maker");
    assert_eq!(&escrow_data[129..137], &1337u64.to_le_bytes(), "receive");
    assert_eq!(escrow_data[137], escrow_bump, "bump");

    println!("  MAKE CU: {}", result.compute_units_consumed);
}

#[test]
fn test_take() {
    let mut svm = setup();

    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;
    let maker = Pubkey::new_unique();
    let taker = Pubkey::new_unique();
    let mint_a = Pubkey::new_unique();
    let mint_b = Pubkey::new_unique();
    let taker_ta_a = Pubkey::new_unique();
    let taker_ta_b = Pubkey::new_unique();
    let maker_ta_b = Pubkey::new_unique();
    let vault_ta_a = Pubkey::new_unique();
    let (escrow, escrow_bump) =
        Pubkey::find_program_address(&[b"escrow", maker.as_ref()], &crate::ID);
    let rent = quasar_svm::solana_sdk_ids::sysvar::rent::ID;

    let data = build_take_data();

    let instruction = with_signers(
        Instruction {
            program_id: crate::ID,
            accounts: vec![
                solana_instruction::AccountMeta::new(taker.into(), true),
                solana_instruction::AccountMeta::new(escrow.into(), false),
                solana_instruction::AccountMeta::new(maker.into(), false),
                solana_instruction::AccountMeta::new_readonly(mint_a.into(), false),
                solana_instruction::AccountMeta::new_readonly(mint_b.into(), false),
                solana_instruction::AccountMeta::new(taker_ta_a.into(), false),
                solana_instruction::AccountMeta::new(taker_ta_b.into(), false),
                solana_instruction::AccountMeta::new(maker_ta_b.into(), false),
                solana_instruction::AccountMeta::new(vault_ta_a.into(), false),
                solana_instruction::AccountMeta::new_readonly(rent.into(), false),
                solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
                solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
            ],
            data,
        },
        &[5, 7], // taker_ta_a, maker_ta_b as signers for create_account CPI
    );

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(taker),
            escrow_account(escrow, maker, mint_a, mint_b, maker_ta_b, 1337, escrow_bump),
            signer(maker),
            mint(mint_a, maker),
            mint(mint_b, maker),
            empty(taker_ta_a),
            token(taker_ta_b, mint_b, taker, 10_000),
            empty(maker_ta_b),
            token(vault_ta_a, mint_a, escrow, 1337),
        ],
    );

    assert!(result.is_ok(), "take failed: {:?}", result.raw_result);
    println!("  TAKE CU: {}", result.compute_units_consumed);
}

#[test]
fn test_refund() {
    let mut svm = setup();

    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;
    let maker = Pubkey::new_unique();
    let mint_a = Pubkey::new_unique();
    let mint_b = Pubkey::new_unique();
    let maker_ta_a = Pubkey::new_unique();
    let maker_ta_b = Pubkey::new_unique();
    let vault_ta_a = Pubkey::new_unique();
    let (escrow, escrow_bump) =
        Pubkey::find_program_address(&[b"escrow", maker.as_ref()], &crate::ID);
    let rent = quasar_svm::solana_sdk_ids::sysvar::rent::ID;

    let data = build_refund_data();

    let instruction = with_signers(
        Instruction {
            program_id: crate::ID,
            accounts: vec![
                solana_instruction::AccountMeta::new(maker.into(), true),
                solana_instruction::AccountMeta::new(escrow.into(), false),
                solana_instruction::AccountMeta::new_readonly(mint_a.into(), false),
                solana_instruction::AccountMeta::new(maker_ta_a.into(), false),
                solana_instruction::AccountMeta::new(vault_ta_a.into(), false),
                solana_instruction::AccountMeta::new_readonly(rent.into(), false),
                solana_instruction::AccountMeta::new_readonly(token_program.into(), false),
                solana_instruction::AccountMeta::new_readonly(system_program.into(), false),
            ],
            data,
        },
        &[3], // maker_ta_a as signer for create_account CPI
    );

    let result = svm.process_instruction(
        &instruction,
        &[
            signer(maker),
            escrow_account(escrow, maker, mint_a, mint_b, maker_ta_b, 1337, escrow_bump),
            mint(mint_a, maker),
            empty(maker_ta_a),
            token(vault_ta_a, mint_a, escrow, 1337),
        ],
    );

    assert!(result.is_ok(), "refund failed: {:?}", result.raw_result);
    println!("  REFUND CU: {}", result.compute_units_consumed);
}
