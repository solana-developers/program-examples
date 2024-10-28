use crate::ACCOUNT;
use checking_account_api::ID;
use solana_program::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
};
use steel::*;


pub fn process_accounts(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {

    // unpack 3 accounts exactly or throw an error
    let [signer_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    // verify that first one is a signer
    signer_info.is_signer()?;

    // new account needs to  
    // 1. be empty
    // 2. be writable
    // 3. has seeds
    new_account_info.is_empty()?.is_writable()?.has_seeds(
        &[ACCOUNT],
        &ID
    )?;

    // verify program ID from the instruction
    system_program.is_program(&system_program::ID)?;

    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", new_account_info.key);
    
    Ok(())
}