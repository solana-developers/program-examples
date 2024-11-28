// api/src/sdk.rs
use steel::*;
use crate::prelude::*;

/// Gets the PDA for a user's favorites account
pub fn favorites_pda(user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"favorites", user.as_ref()],
        &crate::ID
    )
}

/// Creates an instruction to set a user's favorites
pub fn set_favorites(
    user: Pubkey,
    number: u64,
    color: String,
    hobbies: Vec<String>
) -> Result<Instruction, ProgramError> {
    // Validate inputs
    if color.len() > STRING_MAX_SIZE {
        return Err(FavoritesError::StringTooLong.into());
    }
    if hobbies.len() > MAX_HOBBIES {
        return Err(FavoritesError::TooManyHobbies.into());
    }
    for hobby in &hobbies {
        if hobby.len() > STRING_MAX_SIZE {
            return Err(FavoritesError::StringTooLong.into());
        }
    }

    // Convert color to fixed-size array
    let mut color_bytes = [0u8; STRING_MAX_SIZE];
    color_bytes[..color.len()].copy_from_slice(color.as_bytes());

    // Convert hobbies to fixed-size arrays
    let mut hobbies_bytes = [[0u8; STRING_MAX_SIZE]; MAX_HOBBIES];
    let mut hobbies_lens = [0u32; MAX_HOBBIES];
    for (i, hobby) in hobbies.iter().enumerate() {
        hobbies_bytes[i][..hobby.len()].copy_from_slice(hobby.as_bytes());
        hobbies_lens[i] = hobby.len() as u32;
    }

    // Create instruction
    Ok(Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(favorites_pda(&user).0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: SetFavoritesArgs {
            number,
            color_len: color.len() as u32,
            color: color_bytes,
            hobbies_count: hobbies.len() as u32,
            hobbies_len: hobbies_lens,
            hobbies: hobbies_bytes,
        }.to_bytes(),
    })
}

// Optional helper functions to read data from the favorites account

/// Converts fixed-size byte array to String, using the length prefix
fn bytes_to_string(bytes: &[u8], len: u32) -> String {
    String::from_utf8_lossy(&bytes[..len as usize]).to_string()
}

/// Helper to decode the favorites account data
pub fn decode_favorites(favorites: &Favorites) -> (u64, String, Vec<String>) {
    let number = favorites.number;
    let color = bytes_to_string(&favorites.color, favorites.color_len);

    let mut hobbies = Vec::new();
    for i in 0..favorites.hobbies_count as usize {
        let hobby = bytes_to_string(
            &favorites.hobbies[i],
            favorites.hobbies_len[i]
        );
        hobbies.push(hobby);
    }

    (number, color, hobbies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_favorites_instruction() {
        let user = Pubkey::new_unique();
        let number = 42;
        let color = "blue".to_string();
        let hobbies = vec!["reading".to_string(), "coding".to_string()];

        let ix = set_favorites(user, number, color, hobbies).unwrap();

        assert_eq!(ix.program_id, crate::ID);
        assert_eq!(ix.accounts.len(), 3);
        assert_eq!(ix.accounts[0].pubkey, user);
        assert!(ix.accounts[0].is_signer);
        assert_eq!(ix.accounts[1].pubkey, favorites_pda(&user).0);
        assert!(!ix.accounts[1].is_signer);
        assert_eq!(ix.accounts[2].pubkey, system_program::ID);
        assert!(!ix.accounts[2].is_signer);
    }

    #[test]
    fn test_input_validation() {
        let user = Pubkey::new_unique();
        let number = 42;

        // Test color too long
        let color = "x".repeat(STRING_MAX_SIZE + 1);
        let hobbies = vec!["reading".to_string()];
        assert!(set_favorites(user, number, color, hobbies).is_err());

        // Test too many hobbies
        let color = "blue".to_string();
        let hobbies = (0..MAX_HOBBIES + 1)
            .map(|i| format!("hobby{}", i))
            .collect();
        assert!(set_favorites(user, number, color, hobbies).is_err());

        // Test hobby too long
        let color = "blue".to_string();
        let hobbies = vec!["x".repeat(STRING_MAX_SIZE + 1)];
        assert!(set_favorites(user, number, color, hobbies).is_err());
    }
}