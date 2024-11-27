use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};
use steel::{ProgramError, *};

pub fn close_user(accounts: &[AccountInfo]) -> ProgramResult {
    let [payer, target_account, _] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer.is_signer()?;

    steel::close_account(target_account, payer)
}
