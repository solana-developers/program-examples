use account_data_steel_program::AddressInfo;
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

#[test]
fn test_account_data() {
    let mut svm = LiteSVM::new();

    let address_info_account = Keypair::new();
    let payer = Keypair::new();
    let program_id = Pubkey::new_unique();

    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 10)
        .unwrap();

    let program_bytes = include_bytes!("../../tests/fixtures/account_data_steel_program.so");

    svm.add_program(program_id, program_bytes).unwrap();

    let accounts = vec![
        AccountMeta::new(address_info_account.pubkey(), true),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(solana_system_interface::program::ID, false),
    ];

    let mut name = [0u8; 32];
    let name_bytes = b"Joe C";
    name[..name_bytes.len()].copy_from_slice(name_bytes);

    let mut street = [0u8; 32];
    let street_bytes = b"Mile High Dr.";
    street[..street_bytes.len()].copy_from_slice(street_bytes);

    let mut city = [0u8; 32];
    let city_bytes = b"Solana Beach";
    city[..city_bytes.len()].copy_from_slice(city_bytes);

    let data = AddressInfo {
        name,
        house_number: 136,
        street,
        city,
    };

    let mut ix_data = vec![0u8]; // Discriminator
    ix_data.extend_from_slice(bytemuck::bytes_of(&data));

    let ix = Instruction {
        program_id,
        accounts,
        data: ix_data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &address_info_account],
        svm.latest_blockhash(),
    );

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let address_info_account_data = &svm
        .get_account(&address_info_account.pubkey())
        .unwrap()
        .data;

    // Skip discriminator (first 8 bytes)
    let stored_data =
        bytemuck::from_bytes::<AddressInfo>(&address_info_account_data[8..8 + std::mem::size_of::<AddressInfo>()]);

    let name_str = String::from_utf8_lossy(&stored_data.name)
        .trim_matches('\0')
        .to_string();
    let street_str = String::from_utf8_lossy(&stored_data.street)
        .trim_matches('\0')
        .to_string();
    let city_str = String::from_utf8_lossy(&stored_data.city)
        .trim_matches('\0')
        .to_string();

    assert_eq!(name_str, "Joe C");
    assert_eq!(stored_data.house_number, 136);
    assert_eq!(street_str, "Mile High Dr.");
    assert_eq!(city_str, "Solana Beach");
}
