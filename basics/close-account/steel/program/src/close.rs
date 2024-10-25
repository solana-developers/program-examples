use solana_program::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
};
use steel::*;


pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {

    // unpack accounts 
    let [signer_info, account_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    // validate
    signer_info.is_signer()?;
    account_info.is_writable()?;
    
    msg!("Program invoked. Closing a system account...");
    
    close_account(account_info, signer_info)?;
    Ok(())
}