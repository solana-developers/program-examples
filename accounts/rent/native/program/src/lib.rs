use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
    sysvar::Sysvar,
};


entrypoint!(process_instruction);


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    
    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", &new_account.key.to_string());

    // Determine the necessary minimum rent by calculating the account's size
    //
    let account_span = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(AddressData::from_le_bytes)
        .ok_or(ProgramError::InvalidAccountData)?;

    let lamports_required = (Rent::get()?).minimum_balance(account_span as usize);
    
    invoke(
        &system_instruction::create_account(
            &payer.key,
            &new_account.key,
            lamports_required,
            account_span,
            &system_program::ID,
        ),
        &[
            payer.clone(), new_account.clone(), system_program.clone()
        ]
    )?;

    msg!("Account created succesfully.");
    Ok(())
}


// Say this is the data structure we intend our account to have
//
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AddressData {
    name: String,
    address: String,
}