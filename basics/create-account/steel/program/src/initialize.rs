use create_account_api::prelude::*;
use solana_program::msg;
use steel::sysvar::rent::Rent;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Load accounts
    let [payer, new_account, system_program] = accounts else {
        msg!("Not enough accounts provided");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    payer
        .is_signer()
        .map_err(|_| ProgramError::MissingRequiredSignature)?;

    new_account.is_signer()?.is_empty()?.is_writable()?;

    system_program.is_program(&system_program::ID)?;

    // The helper "create_account" will create an account
    // owned by our program and not the system program

    // Calculate the minimum balance needed for the account
    // Space required is the size of our NewAccount struct
    let space_required = std::mem::size_of::<NewAccount>() as u64;
    let lamports_required = (Rent::get()?).minimum_balance(space_required as usize);

    // Create the account by invoking a create_account system instruction
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            payer.key,
            new_account.key,
            lamports_required,
            space_required,
            &system_program::ID,
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
    )?;

    msg!("A new account has been created and initialized!");

    Ok(())
}
