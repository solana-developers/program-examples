use {
    anchor_lang::{
        solana_program::{instruction::Instruction, system_program},
        AnchorSerialize, InstructionData, ToAccountMetas,
    },
    borsh::BorshDeserialize,
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

/// Deserialize the AddressInfo account (8-byte discriminator + fields).
#[derive(BorshDeserialize)]
struct AddressInfoAccount {
    _discriminator: [u8; 8],
    name: String,
    house_number: u8,
    street: String,
    city: String,
}

#[test]
fn test_create_address_info() {
    let program_id = account_data_anchor_program::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/account_data_anchor_program.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let address_info_keypair = Keypair::new();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &account_data_anchor_program::instruction::CreateAddressInfo {
            name: "Joe C".to_string(),
            house_number: 136,
            street: "Mile High Dr.".to_string(),
            city: "Solana Beach".to_string(),
        }
        .data(),
        account_data_anchor_program::accounts::CreateAddressInfo {
            payer: payer.pubkey(),
            address_info: address_info_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(
        &mut svm,
        vec![instruction],
        &[&payer, &address_info_keypair],
        &payer.pubkey(),
    )
    .unwrap();

    // Read the account data back
    let account = svm.get_account(&address_info_keypair.pubkey()).unwrap();
    let info = AddressInfoAccount::deserialize(&mut &account.data[..]).unwrap();

    assert_eq!(info.name, "Joe C");
    assert_eq!(info.house_number, 136);
    assert_eq!(info.street, "Mile High Dr.");
    assert_eq!(info.city, "Solana Beach");
}
