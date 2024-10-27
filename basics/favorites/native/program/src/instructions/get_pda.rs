use solana_program::{
    entrypoint::ProgramResult,
    account_info::{ AccountInfo, next_account_info},
    msg,
    pubkey::Pubkey,
    program_error::ProgramError
};
use borsh::BorshDeserialize;
use crate::state::Favorites;


pub fn get_pda(
    program_id: &Pubkey,   
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?;
    let favorite_account = next_account_info(account_iter)?;

    // deriving the favorite pda 
    let (favorite_pda, _) = Pubkey::find_program_address(&[b"favorite", user.key.as_ref()], program_id);

    // Checking if the favorite account is same as the derived favorite pda
    if favorite_account.key != &favorite_pda {
      return Err(ProgramError::IncorrectProgramId);
    };
  
    let favorites = Favorites::try_from_slice(&favorite_account.data.borrow())?;

    msg!("User {}'s favorite number is {}, favorite color is: {}, and their hobbies are {:#?}", user.key, favorites.number, favorites.color, favorites.hobbies);
    Ok(())
}