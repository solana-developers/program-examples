use create_account_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // validate accounts
    let [signer_info, new_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    new_account
        .is_empty()?
        .is_writable()?
        .has_seeds(&[CREATE_ACCOUNT], &create_account_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Create the account.
    create_account::<NewAccount>(
        new_account,
        system_program,
        signer_info,
        &create_account_api::ID,
        &[CREATE_ACCOUNT],
    )?;

    let created_account = new_account.as_account_mut::<NewAccount>(&create_account_api::ID)?;

    created_account.user_id = 1;

    msg!("A new account has been created and initialized!");
    msg!("Your user id is: {}", created_account.user_id);

    Ok(())
}
