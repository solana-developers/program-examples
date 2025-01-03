use close_account_api::prelude::*;
use steel::*;

pub fn process_create_user(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = CreateUser::try_from_bytes(data)?;
    // let name = String::from_utf8_lossy(&args.name[..])
    //     .trim_end_matches('\0')
    //     .to_string();
    // Load accounts.
    let [user_info, user_state_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    user_info.is_signer()?;
    user_state_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[USER_SEED, user_info.key.as_ref()], &close_account_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize user state.
    create_account::<UserState>(
        user_state_info,
        system_program,
        user_info,
        &close_account_api::ID,
        &[USER_SEED, user_info.key.as_ref()],
    )?;
    let user_state = user_state_info.as_account_mut::<UserState>(&close_account_api::ID)?;
    let user_state_bump = user_state_pda(*user_info.key).1;
    user_state.user = *user_info.key;
    user_state.name = args.name;
    user_state.bump = user_state_bump;

    Ok(())
}
