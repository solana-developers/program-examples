use solana_program::{msg, program::invoke};
use spl_token_2022::extension::transfer_fee::instruction::withdraw_withheld_tokens_from_mint;
use steel::*;

//You can either withdraw tokens from the mint account after "harvesting" or withdraw from the user accounts directly(method: withdraw_withheld_tokens_from_account()).
pub fn process_withdraw(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    //Load accounts
    let [signer_info, mint_info, destination, system_program, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    mint_info.is_writable()?;
    destination.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;

    invoke(
        &withdraw_withheld_tokens_from_mint(
            token_program.key,
            mint_info.key,
            destination.key,
            signer_info.key,
            &[],
        )?,
        &[mint_info.clone(), destination.clone(), signer_info.clone()],
    )?;

    msg!("Transfer Fee Extension: Withdraw successful");
    Ok(())
}
