use std::ffi::CStr;

use solana_program::msg;
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let favorites_data = bytemuck::try_from_bytes::<Favorites>(data)
        .or(Err(ProgramError::InvalidInstructionData))?;

    let [favorites_info, user, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    favorites_info.is_writable()?;
    favorites_info.has_seeds(&[FAVORITES_SEED, user.key.as_ref()], &crate::ID)?;

    // if we have not created our favourites account, let us create it
    if favorites_info.lamports() == 0 {
        create_account::<Favorites>(
            favorites_info,
            system_program,
            user,
            &crate::ID,
            &[FAVORITES_SEED, user.key.as_ref()],
        )?;
    }

    let favorites = favorites_info.as_account_mut::<Favorites>(&crate::ID)?;

    *favorites = *favorites_data;

    let favorite_number = favorites.number;

    msg!("Favorite number: {}", favorite_number);
    msg!(
        "Favorite color: {:?}",
        CStr::from_bytes_until_nul(&favorites.color).unwrap()
    );
    for i in 0..favorites.hobbies.len() {
        msg!(
            "Favorite hobby {i}: {:?}",
            CStr::from_bytes_until_nul(&favorites.hobbies[i]).unwrap()
        );
    }

    Ok(())
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    Favorites = 0,
}

account!(SteelAccount, Favorites);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Favorites {
    pub number: u64,
    pub color: [u8; 48],
    pub hobbies: [[u8; 48]; 5],
}

pub const FAVORITES_SEED: &[u8] = b"favorites";
