use steel::*;
use sysvar::rent::Rent;

pub fn process_create_system_account(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer_info, new_account_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    payer_info.is_signer()?;

    new_account_info.is_signer()?.is_empty()?;

    // Create account.
    let lamports = (Rent::get()?).minimum_balance(0);
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            payer_info.key,
            new_account_info.key,
            lamports,
            0,
            &system_program::ID,
        ),
        &[
            payer_info.clone(),
            new_account_info.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}
