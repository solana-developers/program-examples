use close_account_api::prelude::*;
use close_account_api::ID;
use solana_program::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
};
use steel::*;


pub fn process_init(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {

    // unpack accounts 
    let [signer_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    // validate
    signer_info.is_signer()?;
    new_account_info.is_empty()?.is_writable()?.has_seeds(
        &[ACCOUNT],
        &ID
    )?;
    system_program.is_program(&system_program::ID)?;

    let args = InitializeAccount::try_from_bytes(data)?;
    
    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", new_account_info.key);
    
    create_account::<Data>(
        new_account_info,
        system_program,
        signer_info,
        &close_account_api::ID,
        &[ACCOUNT],
    )?;

    let data_account = new_account_info.as_account_mut::<Data>(&close_account_api::ID)?;
    data_account.name = args.name;

    Ok(())
}