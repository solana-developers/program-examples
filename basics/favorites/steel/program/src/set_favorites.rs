use solana_program::msg;
use steel::*;
use steel_api::prelude::*;

pub fn process_set_favorites(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = SetFavorites::try_from_bytes(data)?;
    let number = u64::from_le_bytes(args.number);
    let color = bytes32_to_string(&args.color).unwrap();
    let hobbies = bytes32_array_to_strings(&args.hobbies).unwrap();

    // Get expected pda bump.
    let favorites_bump = favorites_pda().1;

    // Load accounts.
    let [user_info, favorites_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    user_info.is_signer()?;
    favorites_info.is_empty()?.is_writable()?.has_seeds(
        &[FAVORITES],
        favorites_bump,
        &steel_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize favorites.
    create_account::<Favorites>(
        favorites_info,
        &steel_api::ID,
        &[FAVORITES, &[favorites_bump]],
        system_program,
        user_info,
    )?;

    msg!("Greetings from {}", &steel_api::ID);
    let user_public_key = user_info.key;

    msg!(
              "User {user_public_key}'s favorite number is {number}, favorite color is: {color}, and their hobbies are {hobbies:?}",
          );

    let favorites = favorites_info.to_account_mut::<Favorites>(&steel_api::ID)?;
    favorites.number = number;
    favorites.color = args.color;
    favorites.hobbies = args.hobbies;

    Ok(())
}
