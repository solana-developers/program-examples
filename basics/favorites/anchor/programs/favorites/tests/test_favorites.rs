use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_kite::{create_wallet, send_transaction_from_instructions},
    solana_signer::Signer,
};

struct FavoritesData {
    number: u64,
    color: String,
    hobbies: Vec<String>,
}

/// Manually deserialize the Favorites account data, skipping the 8-byte discriminator.
/// We can't use BorshDeserialize on the full account because init_if_needed allocates
/// more space than the serialized data occupies (padding for max_len strings/vecs).
fn read_favorites(svm: &LiteSVM, pda: &Pubkey) -> FavoritesData {
    let account = svm.get_account(pda).unwrap();
    let data = &account.data[8..]; // skip discriminator
    let mut offset = 0;

    // u64 number
    let number = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
    offset += 8;

    // String color (4-byte length prefix + utf8 data)
    let color_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
    offset += 4;
    let color = String::from_utf8(data[offset..offset + color_len].to_vec()).unwrap();
    offset += color_len;

    // Vec<String> hobbies (4-byte vec length + each string)
    let hobbies_count =
        u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
    offset += 4;
    let mut hobbies = Vec::with_capacity(hobbies_count);
    for _ in 0..hobbies_count {
        let hobby_len =
            u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let hobby = String::from_utf8(data[offset..offset + hobby_len].to_vec()).unwrap();
        offset += hobby_len;
        hobbies.push(hobby);
    }

    FavoritesData {
        number,
        color,
        hobbies,
    }
}

fn setup() -> (LiteSVM, Pubkey, solana_keypair::Keypair) {
    let program_id = favorites::id();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/favorites.so");
    svm.add_program(program_id, bytes).unwrap();
    let payer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

fn favorites_pda(program_id: &Pubkey, user: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"favorites", user.as_ref()], program_id).0
}

#[test]
fn test_set_favorites() {
    let (mut svm, program_id, payer) = setup();
    let pda = favorites_pda(&program_id, &payer.pubkey());

    let instruction = Instruction::new_with_bytes(
        program_id,
        &favorites::instruction::SetFavorites {
            number: 23,
            color: "purple".to_string(),
            hobbies: vec![
                "skiing".to_string(),
                "skydiving".to_string(),
                "biking".to_string(),
            ],
        }
        .data(),
        favorites::accounts::SetFavorites {
            user: payer.pubkey(),
            favorites: pda,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );

    send_transaction_from_instructions(&mut svm, vec![instruction], &[&payer], &payer.pubkey())
        .unwrap();

    let data = read_favorites(&svm, &pda);
    assert_eq!(data.number, 23);
    assert_eq!(data.color, "purple");
    assert_eq!(data.hobbies, vec!["skiing", "skydiving", "biking"]);
}

#[test]
fn test_update_favorites() {
    let (mut svm, program_id, payer) = setup();
    let pda = favorites_pda(&program_id, &payer.pubkey());

    // Set initial favorites
    let instruction = Instruction::new_with_bytes(
        program_id,
        &favorites::instruction::SetFavorites {
            number: 23,
            color: "purple".to_string(),
            hobbies: vec![
                "skiing".to_string(),
                "skydiving".to_string(),
                "biking".to_string(),
            ],
        }
        .data(),
        favorites::accounts::SetFavorites {
            user: payer.pubkey(),
            favorites: pda,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![instruction], &[&payer], &payer.pubkey())
        .unwrap();

    // Update favorites with new hobbies
    svm.expire_blockhash();
    let update_ix = Instruction::new_with_bytes(
        program_id,
        &favorites::instruction::SetFavorites {
            number: 23,
            color: "purple".to_string(),
            hobbies: vec![
                "skiing".to_string(),
                "skydiving".to_string(),
                "biking".to_string(),
                "swimming".to_string(),
            ],
        }
        .data(),
        favorites::accounts::SetFavorites {
            user: payer.pubkey(),
            favorites: pda,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    );
    send_transaction_from_instructions(&mut svm, vec![update_ix], &[&payer], &payer.pubkey())
        .unwrap();

    let data = read_favorites(&svm, &pda);
    assert_eq!(data.number, 23);
    assert_eq!(data.color, "purple");
    assert_eq!(
        data.hobbies,
        vec!["skiing", "skydiving", "biking", "swimming"]
    );
}
