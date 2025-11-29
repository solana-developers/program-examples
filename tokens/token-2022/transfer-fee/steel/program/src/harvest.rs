use solana_program::{msg, program::invoke};
use spl_token_2022::extension::transfer_fee::instruction::harvest_withheld_tokens_to_mint;
use steel::*;

pub fn process_harvest(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    //Load accounts
    let [signer_info, mint_info, harvest_acc_1_info, harvest_acc_2_info, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    mint_info.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;
    harvest_acc_1_info.is_writable()?;
    harvest_acc_2_info.is_writable()?;

    invoke(
        &harvest_withheld_tokens_to_mint(
            token_program.key,
            mint_info.key,
            &[&harvest_acc_1_info.key, &harvest_acc_2_info.key],
        )?,
        &[
            mint_info.clone(),
            harvest_acc_1_info.clone(),
            harvest_acc_2_info.clone(),
        ],
    )?;

    msg!("Transfer Fee Extension: Harvest successful");

    Ok(())
}
