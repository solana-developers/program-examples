use lever_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [user_info, power_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    user_info.is_signer()?;
    power_info.is_signer()?.is_empty()?.is_writable()?;
    system_program.is_program(&system_program::ID)?;

    // Initialize power.
    create_account::<PowerStatus>(power_info, system_program, user_info, &lever_api::ID, &[])?;
    let power = power_info.as_account_mut::<PowerStatus>(&lever_api::ID)?;
    power.is_on = 0;

    Ok(())
}
