use hand_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, power_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    power_info.is_empty()?.is_writable()?;
    system_program.is_program(&system_program::ID)?;

    // Initialize power.
    create_account::<PowerStatus>(power_info, system_program, signer_info, &hand_api::ID, &[])?;

    Ok(())
}
