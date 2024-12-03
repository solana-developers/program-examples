use close_account_api::prelude::*;
use steel::*;

pub fn process_close_user(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [user_info, user_state_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    user_info.is_signer()?;

    let _user_state = user_state_info
        .has_seeds(&[USER_SEED, user_info.key.as_ref()], &close_account_api::ID)?
        .as_account_mut::<UserState>(&close_account_api::ID)?;

    user_state_info.close(user_info)?;

    Ok(())
}
