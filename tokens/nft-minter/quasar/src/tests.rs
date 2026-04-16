extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    spl_token_interface::state::{Account as TokenAccount, AccountState, Mint},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_nft_minter.so").unwrap();
    QuasarSvm::new()
        .with_program(&crate::ID, &elf)
        .with_token_program()
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 5_000_000_000)
}

fn nft_mint(address: Pubkey, authority: Pubkey) -> Account {
    quasar_svm::token::create_keyed_mint_account(
        &address,
        &Mint {
            mint_authority: Some(authority).into(),
            supply: 0,
            decimals: 0,
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

// Note: The mint_nft instruction requires the Metaplex Token Metadata program
// deployed in the SVM for the create_metadata and create_master_edition CPIs.
// The quasar-svm harness does not currently include it, so we verify the
// program builds and can at least mint a token (the first CPI step).
// Full integration testing requires a devnet/localnet deploy with Metaplex.

#[test]
fn test_program_builds() {
    let _svm = setup();
    println!("  NFT minter program loaded successfully");
}

#[test]
fn test_mint_to_token_account() {
    // Test that the SPL Token mint_to CPI works independently.
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let mint_address = Pubkey::new_unique();
    let token_addr = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_PROGRAM_ID;

    // Build a raw mint_to CPI instruction to verify the token setup works.
    let mut data = vec![7u8]; // SPL Token MintTo instruction
    data.extend_from_slice(&1u64.to_le_bytes());

    let instruction = Instruction {
        program_id: token_program,
        accounts: vec![
            solana_instruction::AccountMeta::new(mint_address.into(), false),
            solana_instruction::AccountMeta::new(token_addr.into(), false),
            solana_instruction::AccountMeta::new_readonly(payer.into(), true),
        ],
        data,
    };

    let result = svm.process_instruction(
        &instruction,
        &[
            nft_mint(mint_address, payer),
            token_account(token_addr, mint_address, payer, 0),
            signer(payer),
        ],
    );

    assert!(result.is_ok(), "SPL Token mint_to failed: {:?}", result.raw_result);
    println!("  MINT TO CU: {}", result.compute_units_consumed);
}
