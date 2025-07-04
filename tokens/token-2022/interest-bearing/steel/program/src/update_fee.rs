use solana_program::{msg, program::invoke};
use spl_token_2022::extension::interest_bearing_mint;
use steel::*;
use steel_api::prelude::*;

pub fn process_update_rate(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = UpdateRate::try_from_bytes(&data)?;
    let rate = i16::from_le_bytes(args.rate);

    // Load accounts.
    let [signer_info, mint_info, system_program, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token_2022::ID)?;

    invoke(
        &interest_bearing_mint::instruction::update_rate(
            token_program.key,
            mint_info.key,
            &signer_info.key,
            &[],
            rate,
        )?,
        &[mint_info.clone(), signer_info.clone()],
    )?;

    msg!("Interest Bearing Mint Extension: Rate Updated!");

    Ok(())
}
