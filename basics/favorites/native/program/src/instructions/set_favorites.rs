use crate::state::Favorites;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn set_favorites(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    number: u64,
    color: String,
    hobbies: Vec<String>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let favorites_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) =
        Pubkey::find_program_address(&[b"favorites", user_account.key.as_ref()], program_id);

    if pda != *favorites_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    let favorites = Favorites {
        number,
        color: color.clone(),
        hobbies: hobbies.clone(),
    };

    let serialized_data = favorites.try_to_vec()?;
    let space_needed = serialized_data.len();
    let rent = Rent::get()?;
    let lamports_needed = rent.minimum_balance(space_needed);

    if favorites_account.lamports() == 0 {
        let ix = system_instruction::create_account(
            user_account.key,
            favorites_account.key,
            lamports_needed,
            space_needed as u64,
            program_id,
        );
        solana_program::program::invoke_signed(
            &ix,
            &[
                user_account.clone(),
                favorites_account.clone(),
                system_program.clone(),
            ],
            &[&[b"favorites", user_account.key.as_ref(), &[bump_seed]]],
        )?;
    } else {
        let current_space = favorites_account.data_len();
        if space_needed > current_space {
            let additional_rent = lamports_needed.saturating_sub(favorites_account.lamports());
            if additional_rent > 0 {
                let transfer_ix = system_instruction::transfer(
                    user_account.key,
                    favorites_account.key,
                    additional_rent,
                );
                solana_program::program::invoke(
                    &transfer_ix,
                    &[
                        user_account.clone(),
                        favorites_account.clone(),
                        system_program.clone(),
                    ],
                )?;
            }
            favorites_account.realloc(space_needed, false)?;
        }
    }
    favorites.serialize(&mut &mut favorites_account.data.borrow_mut()[..])?;
    msg!("User's favorite number is: {}", number);
    msg!("User's favorite color is: {}", color);
    msg!("User's hobbies are: {:?}", hobbies);

    Ok(())
}
