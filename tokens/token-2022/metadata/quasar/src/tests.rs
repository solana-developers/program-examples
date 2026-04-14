extern crate std;
use {
    alloc::vec,
    quasar_svm::{Account, Instruction, Pubkey, QuasarSvm},
    std::println,
};

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/quasar_token_2022_metadata.so").unwrap();
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
fn test_initialize() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let token_program = quasar_svm::SPL_TOKEN_2022_PROGRAM_ID;
    let system_program = quasar_svm::system_program::ID;

    let name = b"Test Token";
    let symbol = b"TEST";
    let uri = b"https://example.com/token.json";

    // Encode args to match function signature: fixed-size padded arrays + u8 lengths
    // name: [u8; MAX_NAME=32], name_len: u8, symbol: [u8; MAX_SYMBOL=10], symbol_len: u8,
    // uri: [u8; MAX_URI=128], uri_len: u8
    let mut data = vec![0u8]; // discriminator = 0
    let mut name_fixed = [0u8; 32];
    name_fixed[..name.len()].copy_from_slice(name);
    data.extend_from_slice(&name_fixed);
    data.push(name.len() as u8);
    let mut symbol_fixed = [0u8; 10];
    symbol_fixed[..symbol.len()].copy_from_slice(symbol);
    data.extend_from_slice(&symbol_fixed);
    data.push(symbol.len() as u8);
    let mut uri_fixed = [0u8; 128];
    uri_fixed[..uri.len()].copy_from_slice(uri);
    data.extend_from_slice(&uri_fixed);
    data.push(uri.len() as u8);

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
