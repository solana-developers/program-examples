use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_account_data.so");
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

/// Build create_address_info instruction data manually.
///
/// Wire format (from reading the #[instruction] codegen):
///   [disc: 1 byte]
///   [ZC struct: house_number u8]
///   [name: u32 LE length prefix + bytes]  (String → DynKind::Str with U32 prefix)
///   [street: u32 LE length prefix + bytes]
///   [city: u32 LE length prefix + bytes]
fn build_create_instruction_data(name: &str, house_number: u8, street: &str, city: &str) -> Vec<u8> {
    let mut data = vec![0u8]; // discriminator = 0

    // Fixed ZC struct: house_number
    data.push(house_number);

    // Dynamic String args with u32 length prefix
    data.extend_from_slice(&(name.len() as u32).to_le_bytes());
    data.extend_from_slice(name.as_bytes());

    data.extend_from_slice(&(street.len() as u32).to_le_bytes());
    data.extend_from_slice(street.as_bytes());

    data.extend_from_slice(&(city.len() as u32).to_le_bytes());
    data.extend_from_slice(city.as_bytes());

    data
}

#[test]
fn test_create_address_info() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;

    let (address_info, _) = Pubkey::find_program_address(
        &[b"address_info", payer.as_ref()],
        &Pubkey::from(crate::ID),
    );

    let data = build_create_instruction_data("Alice", 42, "Main Street", "New York");

    let instruction = Instruction {
        program_id: Pubkey::from(crate::ID),
        accounts: vec![
            solana_instruction::AccountMeta::new(Address::from(payer.to_bytes()), true),
            solana_instruction::AccountMeta::new(Address::from(address_info.to_bytes()), false),
            solana_instruction::AccountMeta::new_readonly(
                Address::from(system_program.to_bytes()),
                false,
            ),
        ],
        data,
    };

    let result = svm.process_instruction(&instruction, &[signer(payer), empty(address_info)]);

    result.assert_success();

    // Verify the account data.
    let account = result.account(&address_info).unwrap();

    // On-chain layout (from #[account] dynamic codegen):
    //   [disc: 1 byte = 1]
    //   [ZC header: house_number u8]
    //   [name: u8 prefix + bytes]   (String<u8, 50> uses u8 prefix)
    //   [street: u8 prefix + bytes]
    //   [city: u8 prefix + bytes]
    assert_eq!(account.data[0], 1, "discriminator");
    assert_eq!(account.data[1], 42, "house_number");

    let mut offset = 2;

    // name: u8 prefix + "Alice"
    let name_len = account.data[offset] as usize;
    offset += 1;
    assert_eq!(name_len, 5);
    assert_eq!(&account.data[offset..offset + name_len], b"Alice");
    offset += name_len;

    // street: u8 prefix + "Main Street"
    let street_len = account.data[offset] as usize;
    offset += 1;
    assert_eq!(street_len, 11);
    assert_eq!(&account.data[offset..offset + street_len], b"Main Street");
    offset += street_len;

    // city: u8 prefix + "New York"
    let city_len = account.data[offset] as usize;
    offset += 1;
    assert_eq!(city_len, 8);
    assert_eq!(&account.data[offset..offset + city_len], b"New York");
}
