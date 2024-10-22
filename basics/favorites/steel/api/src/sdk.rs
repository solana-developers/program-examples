use steel::*;

use crate::prelude::*;

pub fn set_favorites(
    signer: Pubkey,
    number: u64,
    color: String,
    hobbies: Vec<String>,
) -> Instruction {
    let color_bytes: [u8; 64] = color
        .as_bytes()
        .try_into()
        .expect("String wrong length, expected 32 bytes");

    let hobbies_bytes = convert_vec_to_byte_arrays(hobbies);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(favorites_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: SetFavorites {
            number: number.to_le_bytes(),
            color: color_bytes,
            hobbies: hobbies_bytes,
        }
        .to_bytes(),
    }
}

fn convert_vec_to_byte_arrays(strings: Vec<String>) -> [[u8; 64]; 5] {
    let mut result = [[0u8; 64]; 5];

    // Ensure we have exactly 5 strings
    assert_eq!(strings.len(), 5, "Expected exactly 5 strings");

    // Convert each string to a fixed-size byte array
    for (i, string) in strings.into_iter().enumerate() {
        result[i] = string.as_bytes().try_into().expect(&format!(
            "String at index {} wrong length, expected 64 bytes",
            i
        ));
    }

    result
}
