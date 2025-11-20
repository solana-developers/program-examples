use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::default_account_state::instruction::update_default_account_state,
    state::AccountState,
};
use steel::*;

pub fn process_update_default_state(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, mint_info, system_program, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //Validation
    signer_info.is_signer()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Update Default Account State to Initialized
    invoke(
        &update_default_account_state(
            token_program.key,
            mint_info.key,
            signer_info.key,
            &[],
            &AccountState::Initialized,
        )?,
        &[mint_info.clone(), signer_info.clone()],
    )?;

    msg!("Default Account State Extension: State Updated.");

    Ok(())
}
