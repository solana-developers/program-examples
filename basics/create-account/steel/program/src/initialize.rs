use create_account_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>]) -> ProgramResult {

    // Validate accounts
    let [payer, new_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    payer.is_signer()?;
    new_account
        .is_empty()?
        .is_writable()?
        .has_seeds(&[CREATE_ACCOUNT, payer.key.as_ref()], &create_account_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize the account.
    create_account::<NewAccount>(
        new_account,
        system_program,
        payer,
        &create_account_api::ID,
        &[CREATE_ACCOUNT, payer.key.as_ref()],
    )?;

    // Fetch the newly created account and give it
    // an arbitrary user id
    let created_account = new_account.as_account_mut::<NewAccount>(&create_account_api::ID)?;
    created_account.user_id = 1;

    msg!("A new account has been created and initialized!");
    msg!("Your user id is: {}", created_account.user_id);

    Ok(())
}
