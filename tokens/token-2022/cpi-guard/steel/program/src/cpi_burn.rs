use solana_program::{msg, program::invoke};
use spl_token_2022::instruction::{burn, mint_to};
use steel::*;

pub fn process_cpi_burn(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, mint_info, recipient_token_account_info, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //Validation
    signer_info.is_signer()?;
    recipient_token_account_info.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;

    invoke(
        &mint_to(
            token_program.key,
            mint_info.key,
            recipient_token_account_info.key,
            signer_info.key,
            &[],
            1000,
        )?,
        &[
            mint_info.clone(),
            recipient_token_account_info.clone(),
            signer_info.clone(),
        ],
    )?;

    invoke(
        &burn(
            token_program.key,
            recipient_token_account_info.key,
            mint_info.key,
            signer_info.key,
            &[],
            100,
        )?,
        &[
            recipient_token_account_info.clone(),
            mint_info.clone(),
            signer_info.clone(),
        ],
    )?;

    msg!("Cpi Guard Extension Test: Burn.");

    Ok(())
}
