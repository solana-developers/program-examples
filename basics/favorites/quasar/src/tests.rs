use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_favorites.so");
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

/// Build set_favorites instruction data.
/// Wire format: [disc=0] [ZC: number(u64)] [color: u32 prefix + bytes]
fn build_set_favorites(number: u64, color: &str) -> Vec<u8> {
    let mut data = vec![0u8]; // discriminator = 0

    // Fixed ZC args: number (u64, but as Pod it's le bytes)
    data.extend_from_slice(&number.to_le_bytes());

    // Dynamic String arg: color with u32 prefix
    data.extend_from_slice(&(color.len() as u32).to_le_bytes());
    data.extend_from_slice(color.as_bytes());

    data
}

#[test]
fn test_set_favorites() {
    let mut svm = setup();

    let user = Pubkey::new_unique();
    let system_program = quasar_svm::system_program::ID;
    let program_id = Pubkey::from(crate::ID);

    let (favorites, _) =
        Pubkey::find_program_address(&[b"favorites", user.as_ref()], &program_id);

    let ix = Instruction {
        program_id,
        accounts: vec![
            solana_instruction::AccountMeta::new(Address::from(user.to_bytes()), true),
            solana_instruction::AccountMeta::new(Address::from(favorites.to_bytes()), false),
            solana_instruction::AccountMeta::new_readonly(
                Address::from(system_program.to_bytes()),
                false,
            ),
        ],
        data: build_set_favorites(42, "blue"),
    };

    let result = svm.process_instruction(&ix, &[signer(user), empty(favorites)]);
    result.assert_success();

    // Verify stored data.
    let account = result.account(&favorites).unwrap();

    // Data layout: [disc(1)] [ZC: number(8 bytes)] [color: u8 prefix + bytes]
    assert_eq!(account.data[0], 1, "discriminator");

    let number = u64::from_le_bytes(account.data[1..9].try_into().unwrap());
    assert_eq!(number, 42, "favourite number");

    // color: u8 prefix at offset 9, then "blue" (4 bytes)
    assert_eq!(account.data[9], 4, "color length");
    assert_eq!(&account.data[10..14], b"blue", "color data");
}
