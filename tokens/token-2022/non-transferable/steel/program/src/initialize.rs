use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::ExtensionType, instruction::initialize_non_transferable_mint, state::Mint,
};
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, mint_info, system_program, token_program, rent_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    msg!("Account loaded");

    //Validation
    signer_info.is_signer()?;
    mint_info.is_signer()?.is_empty()?.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;
    rent_sysvar.is_sysvar(&sysvar::rent::ID)?;

    //Calculate space for mint and extension data
    let space =
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::NonTransferable])?;

    //Create account for the mint and allocate space
    create_account(
        signer_info,
        mint_info,
        system_program,
        space,
        token_program.key,
    )?;

    //Initialize the non transferable extension
    //This instruction must come before the instruction to initialize mint data
    invoke(
        &initialize_non_transferable_mint(token_program.key, mint_info.key)?,
        &[mint_info.clone()],
    )?;

    initialize_mint(
        mint_info,
        signer_info,
        Some(signer_info),
        token_program,
        rent_sysvar,
        6, //you can always pass this as instruction data
    )?;

    msg!("Non Transferable Extension: Initialized.");

    Ok(())
}
