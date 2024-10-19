use counter_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Get expected pda bump.
    let counter_bump = counter_pda().1;

    // Load accounts.
    let [signer_info, counter_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    signer_info.is_signer()?;
    counter_info.is_empty()?.is_writable()?.has_seeds(
        &[COUNTER_SEED],
        counter_bump,
        &counter_api::ID
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize counter.
    create_account::<Counter>(
        counter_info,
        &counter_api::ID,
        &[COUNTER_SEED, &[counter_bump]],
        system_program,
        signer_info,
    )?;
    let counter = counter_info.to_account_mut::<Counter>(&counter_api::ID)?;
    counter.value = 0;

    Ok(())
}
