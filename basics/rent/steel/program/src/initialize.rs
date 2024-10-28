use rent_api::prelude::*;
use rent_api::ID;
use solana_program::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
    rent::Rent,
};
use steel::*;


const MIN_DATA_LENGTH: usize = 96; // 32 for name + 64 for address

pub fn process_initialize_account(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {

    // parse args 
    if data.len() < MIN_DATA_LENGTH {
        return Err(ProgramError::InvalidInstructionData.into());
    }
    let args = InitializeAccount::try_from_bytes(data)?;

    msg!("Args:");
    msg!("  Name: {}", std::str::from_utf8(&args.name).unwrap_or("Invalid UTF-8"));
    msg!("  Address: {}", std::str::from_utf8(&args.address).unwrap_or("Invalid UTF-8"));

    let [signer_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    // validate accounts
    signer_info.is_signer()?;
    new_account_info.is_empty()?.is_writable()?.has_seeds(
        &[ACCOUNT],
        &ID
    )?;
    system_program.is_program(&system_program::ID)?;

    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", new_account_info.key);

    // calculate rent
    let account_span = std::mem::size_of::<Data>();
    let lamports_required = Rent::get()?.minimum_balance(account_span);
    msg!("  Account span: {}", account_span);
    msg!("  Lamports required: {}", lamports_required);

    // create account
    create_account::<Data>(
        new_account_info, 
        system_program, 
        signer_info, 
        &ID, 
        &[ACCOUNT]
    )?;
    
    // set data
    let data_account = new_account_info.as_account_mut::<Data>(&ID)?;

    data_account.name = args.name;
    data_account.address = args.address;
    
    msg!("created and data set successfully.");
    Ok(())
}