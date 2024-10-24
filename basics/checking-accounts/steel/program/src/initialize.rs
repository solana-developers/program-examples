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

    // new account needs to be 
    // 1. empty
    // 2. writable
    new_account_info.is_empty()?.is_writable()?;

    // verify program ID from the instruction
    system_program.is_program(&system_program::ID)?;

    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", new_account_info.key);
    
    Ok(())
}