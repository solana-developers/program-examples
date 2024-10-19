use create_account_api::prelude::*;
use steel::*;
use solana_program::msg;

pub fn process_initialize(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Get expected pda bump.
    let new_account_bump = new_account_pda().1;

    // Load accounts.
    let [signer_info, newaccount, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    signer_info.is_signer()?;
    newaccount.is_empty()?.is_writable()?.has_seeds(
        &[NEWACCOUNT],
        new_account_bump,
        &create_account_api::ID
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize new account.
    create_account::<NewAccount>(
        newaccount,
        &create_account_api::ID,
        &[NEWACCOUNT, &[new_account_bump]],
        system_program,
        signer_info,
    )?;

    // fetch new account

    let new_account = newaccount.to_account_mut::<NewAccount>(&create_account_api::ID)?;
    new_account.userID = 3;

    msg!("Initialized");
    msg!("Initial Value {}", new_account.userID);

    Ok(())
}
