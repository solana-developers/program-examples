use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::ExtensionType, instruction::initialize_permanent_delegate, state::Mint,
};
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, mint_info, delegate_info, system_program, token_program, rent_sysvar] =
        accounts
    else {
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
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::PermanentDelegate])?;

    //Create account for the mint and allocate space
    create_account(
        signer_info,
        mint_info,
        system_program,
        space,
        token_program.key,
    )?;

    //Initialize the permanent delegate extension
    //This instruction must come before the instruction to initialize mint data
    invoke(
        &initialize_permanent_delegate(token_program.key, mint_info.key, delegate_info.key)?,
        &[mint_info.clone()],
    )?;

    initialize_mint(
        mint_info,
        signer_info,
        Some(signer_info),
        token_program,
        rent_sysvar,
        6,
    )?;

    msg!("Permanent Delegate Extension: Initialized.");

    Ok(())
}
