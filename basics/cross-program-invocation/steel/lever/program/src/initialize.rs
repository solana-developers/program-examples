use lever_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let [power_info, user_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    user_info.is_signer()?;
    system_program.is_program(&system_program::ID)?;

    // Create seeds slice properly
    let seeds: &[&[u8]] = &[b"power"];

    // Correct argument order for create_account:
    // create_account<T>(
    //     account_info: &AccountInfo,
    //     program_id: &Pubkey,
    //     seeds: &[&[u8]],
    //     system_program: &AccountInfo,
    //     payer: &AccountInfo,
    // )
    create_account::<PowerStatus>(
        power_info,
        &lever_api::ID,
        seeds,
        system_program,
        user_info,
    )?;

    let power = power_info.to_account_mut::<PowerStatus>(&lever_api::ID)?;
    power.is_on = 0;
    Ok(())
}