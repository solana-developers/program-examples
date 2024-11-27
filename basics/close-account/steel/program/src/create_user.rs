use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

use close_account_api::prelude::{CreateAccount, User};
use steel::{create_account, system_program, AccountInfoValidation, AsAccount, ProgramError};

pub fn create_user(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let value = CreateAccount::try_from_bytes(data)?;

    let [payer, target_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer.is_signer()?;

    target_account.is_empty()?.is_writable()?.has_seeds(
        &[User::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
        &close_account_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;

    create_account::<User>(
        target_account,
        system_program,
        payer,
        &close_account_api::ID,
        &[User::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
    )?;

    let user = target_account.as_account_mut::<User>(&close_account_api::ID)?;
    user.name = value.0;

    Ok(())
}
