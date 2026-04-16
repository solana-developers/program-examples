use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_rent.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
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

/// Build create_system_account instruction data (discriminator = 0).
/// Wire format: [disc=0] [name: String] [address: String]
/// Both String args are dynamic (u32 length prefix + bytes).
fn build_create_system_account(name: &str, address: &str) -> Vec<u8> {
    let mut data = vec![0u8]; // discriminator = 0

    // Dynamic String: name
    data.extend_from_slice(&(name.len() as u32).to_le_bytes());
    data.extend_from_slice(name.as_bytes());

    // Dynamic String: address
    data.extend_from_slice(&(address.len() as u32).to_le_bytes());
    data.extend_from_slice(address.as_bytes());

    data
}

#[test]
fn test_create_system_account_for_address_data() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let new_account = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let name = "Joe C";
    let address = "123 Main St";

    let ix = Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![
            solana_instruction::AccountMeta::new(
                Address::from(payer.to_bytes()),
                true,
            ),
            solana_instruction::AccountMeta::new(
                Address::from(new_account.to_bytes()),
                true,
            ),
            solana_instruction::AccountMeta::new_readonly(
                Address::from(system_program.to_bytes()),
                false,
            ),
        ],
        data: build_create_system_account(name, address),
    };

    let result = svm.process_instruction(
        &ix,
        &[signer(payer), empty(new_account)],
    );

    result.assert_success();

    // Verify the account was created with the expected data size.
    let account = result.account(&new_account).unwrap();
    let expected_space = 4 + name.len() + 4 + address.len();
    assert_eq!(
        account.data.len(),
        expected_space,
        "account data should be sized for the address data"
    );
    assert!(account.lamports > 0, "account should have rent-exempt lamports");

    let logs = result.logs.join("\n");
    assert!(logs.contains("Creating a system account"), "should log creation");
    assert!(logs.contains("Account created successfully"), "should log success");
}
