use borsh::{BorshDeserialize, BorshSerialize};
use std::convert::TryFrom;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FavoritesList {
    pub owner: Pubkey,
    pub accounts: Vec<Pubkey>,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let favorites_account = next_account_info(account_info_iter)?;

    // Parsing the instruction data (the first byte indicates the action: 0 = add, 1 = remove)
    let (action, favorite_pubkey) = instruction_data.split_at(1);
    let favorite_pubkey = Pubkey::try_from(favorite_pubkey).map_err(|_| ProgramError::InvalidArgument)?;

    if action == [0] { // add favorite
        add_favorite(user_account, favorites_account, &favorite_pubkey)?;
    } else if action == [1] { // remove favorite
        remove_favorite(user_account, favorites_account, &favorite_pubkey)?;
    } else {
        return Err(ProgramError::InvalidInstructionData);
    }

    Ok(())
}

fn add_favorite(
    user_account: &AccountInfo,
    favorites_account: &AccountInfo,
    favorite_pubkey: &Pubkey,
) -> ProgramResult {
    let mut favorites_list: FavoritesList = FavoritesList::try_from_slice(&favorites_account.data.borrow())?;
    if favorites_list.owner != *user_account.key {
        return Err(ProgramError::IllegalOwner);
    }

    favorites_list.accounts.push(*favorite_pubkey);
    favorites_list.serialize(&mut &mut favorites_account.data.borrow_mut()[..])?;

    Ok(())
}

fn remove_favorite(
    user_account: &AccountInfo,
    favorites_account: &AccountInfo,
    favorite_pubkey: &Pubkey,
) -> ProgramResult {
    let mut favorites_list: FavoritesList = FavoritesList::try_from_slice(&favorites_account.data.borrow())?;
    if favorites_list.owner != *user_account.key {
        return Err(ProgramError::IllegalOwner);
    }

    // Remove the favorite from the list
    favorites_list.accounts.retain(|&x| x != *favorite_pubkey);
    favorites_list.serialize(&mut &mut favorites_account.data.borrow_mut()[..])?;
    msg!("Favorite removed: {:?}", favorite_pubkey);

    Ok(())
}