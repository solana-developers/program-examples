use account_data_api::prelude::*;
use account_data_api::ID;
use solana_program::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
};
use steel::*;

// instruction data expected length
const MIN_DATA_LENGTH: usize = 64 + 1 + 64 + 64;

pub fn process_initialize_account(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {

    // parse args 
    if data.len() < MIN_DATA_LENGTH {
        return Err(ProgramError::InvalidInstructionData.into());
    }
    let args = InitializeAccount::try_from_bytes(data)?;

    msg!("Args:");
    msg!("  Name: {}", std::str::from_utf8(&args.name).unwrap_or("Invalid UTF-8"));
    msg!("  house_number: {}", args.house_number);
    msg!("  city: {}", std::str::from_utf8(&args.city).unwrap_or("Invalid UTF-8"));
    msg!("  street: {}", std::str::from_utf8(&args.street).unwrap_or("Invalid UTF-8"));

    let [signer_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    let bump = account_pda().1;
    
    // validate accounts
    signer_info.is_signer()?;
    new_account_info.is_empty()?.is_writable()?.has_seeds(
        &[ACCOUNT],
        bump,
        &account_data_api::ID
    )?;
    system_program.is_program(&system_program::ID)?;

    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", new_account_info.key);
    
    // create account
    create_account::<Data>(
        new_account_info,
        &ID,
        &[ACCOUNT, &[bump]],
        system_program,
        signer_info,
    )?;
    
    // set data
    let data_account = new_account_info.to_account_mut::<Data>(&ID)?;
    data_account.name = args.name;
    data_account.house_number = args.house_number;
    data_account.city = args.city;
    data_account.street = args.street;
    
    msg!("created and data set successfully.");
    Ok(())
}