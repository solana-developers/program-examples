use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

/// Lever program discriminator for PowerStatus account (must match
/// #[account(discriminator = 1)]).
const POWER_STATUS_DISCRIMINATOR: u8 = 1;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_lever.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

fn signer(address: Pubkey) -> Account {
    quasar_svm::token::create_keyed_system_account(&address, 10_000_000_000)
}

/// Derive the power PDA address.
fn power_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"power"], &Pubkey::from(crate::ID))
}

fn empty_pda(address: Pubkey) -> Account {
    Account {
        address,
        lamports: 0,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    }
}

fn power_account(address: Pubkey, is_on: bool) -> Account {
    // Account data: [discriminator: u8] [is_on: u8]
    let data = vec![POWER_STATUS_DISCRIMINATOR, if is_on { 1 } else { 0 }];
    Account {
        address,
        lamports: 1_000_000_000,
        data,
        owner: Pubkey::from(crate::ID),
        executable: false,
    }
}

/// Build initialize instruction data (discriminator = 0).
fn build_initialize() -> Vec<u8> {
    vec![0u8]
}

/// Build switch_power instruction data (discriminator = 1).
/// Wire format: [disc=1] [name: String]
fn build_switch_power(name: &str) -> Vec<u8> {
    let mut data = vec![1u8]; // discriminator = 1
    data.extend_from_slice(&(name.len() as u32).to_le_bytes());
    data.extend_from_slice(name.as_bytes());
    data
}

#[test]
fn test_initialize_lever() {
    let mut svm = setup();
    let payer = Pubkey::new_unique();
    let (power_addr, _bump) = power_pda();
    let system_program = quasar_svm::system_program::ID;

    let ix = Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![
            solana_instruction::AccountMeta::new(Address::from(payer.to_bytes()), true),
            solana_instruction::AccountMeta::new(Address::from(power_addr.to_bytes()), false),
            solana_instruction::AccountMeta::new_readonly(
                Address::from(system_program.to_bytes()),
                false,
            ),
        ],
        data: build_initialize(),
    };

    let result = svm.process_instruction(&ix, &[signer(payer), empty_pda(power_addr)]);
    result.assert_success();

    // Power should be off (false) after initialization.
    let account = result.account(&power_addr).unwrap();
    assert_eq!(account.data.len(), 2, "discriminator + is_on");
    assert_eq!(account.data[1], 0, "power should be off initially");
}

#[test]
fn test_switch_power_on() {
    let mut svm = setup();
    let (power_addr, _bump) = power_pda();

    let ix = Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![
            solana_instruction::AccountMeta::new(Address::from(power_addr.to_bytes()), false),
        ],
        data: build_switch_power("Alice"),
    };

    // Start with power off.
    let result = svm.process_instruction(&ix, &[power_account(power_addr, false)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("pulling the power switch"), "should log switch");
    assert!(logs.contains("now on"), "should say power is on");

    let account = result.account(&power_addr).unwrap();
    assert_eq!(account.data[1], 1, "power should now be on");
}

#[test]
fn test_switch_power_off() {
    let mut svm = setup();
    let (power_addr, _bump) = power_pda();

    let ix = Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![
            solana_instruction::AccountMeta::new(Address::from(power_addr.to_bytes()), false),
        ],
        data: build_switch_power("Bob"),
    };

    // Start with power on.
    let result = svm.process_instruction(&ix, &[power_account(power_addr, true)]);
    result.assert_success();

    let logs = result.logs.join("\n");
    assert!(logs.contains("now off"), "should say power is off");

    let account = result.account(&power_addr).unwrap();
    assert_eq!(account.data[1], 0, "power should now be off");
}
