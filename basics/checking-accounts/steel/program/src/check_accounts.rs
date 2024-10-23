use solana_program::msg;
use steel::*;
use steel_api::prelude::*;

pub fn process_check_accounts(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Get expected pda bump.
    let account_to_change_bump = account_to_change_pda().1;

    // Load accounts.
    // You can verify the list has the correct number of accounts.
    let [signer_info, account_to_create_info, account_to_change_info, system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // You can verify if an account is a signer
    signer_info.is_signer()?;

    // You can verify the program ID from the instruction is in fact
    //      the program ID of your program.
    if system_program.is_program(&system_program::ID).is_err() {
        return Err(ProgramError::IncorrectProgramId);
    };

    // You can make sure an account has NOT been initialized.

    msg!("New account: {}", account_to_create_info.key);
    if account_to_create_info.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized);
    };
    // (Create account...)
    create_account::<AccountToChange>(
        account_to_change_info,
        &steel_api::ID,
        &[ACCOUNT_TO_CHANGE, &[account_to_change_bump]],
        system_program,
        signer_info,
    )?;

    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change_info.key);
    if account_to_change_info.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount);
    };

    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change_info.owner != &steel_api::ID {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId);
    };

    // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    };

    Ok(())
}
