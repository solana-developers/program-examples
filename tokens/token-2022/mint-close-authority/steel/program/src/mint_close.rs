use solana_program::{msg, program::invoke};
use spl_token_2022::instruction::close_account;
use steel::*;

pub fn process_mint_close(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, mint_info, destination_info, system_program, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    mint_info.is_signer()?;
    destination_info.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;

    invoke(
        &close_account(
            token_program.key,
            mint_info.key,
            destination_info.key,
            signer_info.key,
            &[],
        )?,
        &[mint_info.clone(), destination_info.clone(), signer_info.clone()],
    )?;

    msg!("Mint Close Authority Extension: Account Closed!");

    Ok(())
}
