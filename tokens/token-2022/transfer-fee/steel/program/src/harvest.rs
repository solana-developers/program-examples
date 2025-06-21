use solana_program::{msg, program::invoke};
use spl_token_2022::extension::transfer_fee::instruction::harvest_withheld_tokens_to_mint;
use steel::*;

pub fn process_harvest(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    //Load accounts
    let [signer_info, mint_info, harvest_acc_1, harvest_acc_2, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    mint_info.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;
    harvest_acc_1.is_writable()?;
    harvest_acc_2.is_writable()?;

    invoke(
        &harvest_withheld_tokens_to_mint(
            token_program.key,
            mint_info.key,
            &[&harvest_acc_1.key, &harvest_acc_2.key],
        )?,
        &[
            mint_info.clone(),
            harvest_acc_1.clone(),
            harvest_acc_2.clone(),
        ],
    )?;

    msg!("Transfer Fee Extension: Harvest successful");

    Ok(())
}
