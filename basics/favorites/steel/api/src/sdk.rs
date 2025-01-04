use steel::*;

use crate::prelude::*;

pub fn set_favorites(signer: Pubkey, number: u64, color: &str, hobbies: Vec<&str>) -> Instruction {
    let color_bytes: [u8; 32] = string_to_bytes32_padded(color).unwrap();

    let hobbies_bytes = strings_to_bytes32_array_padded(hobbies).unwrap();

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
