use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::{transfer_fee::instruction::initialize_transfer_fee_config, ExtensionType},
    state::Mint,
};
use steel::*;
use steel_api::prelude::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    let args = Initialize::try_from_bytes(&_data)?;
    let maximum_fee = u64::from_le_bytes(args.maximum_fee);
    let transfer_fee_basis_points = u16::from_le_bytes(args.transfer_fee_basis_points);
    msg!("Parsed arguments");

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
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::TransferFeeConfig])?;

    //Create account for the mint and allocate space
    create_account(
        signer_info,
        mint_info,
        system_program,
        space,
        token_program.key,
    )?;

    //Initialize the transfer fee extension
    //This instruction must come before the instruction to initialize mint data
    invoke(
        &initialize_transfer_fee_config(
            token_program.key,
            mint_info.key,
            Some(signer_info.key),
            Some(signer_info.key),
            transfer_fee_basis_points,
            maximum_fee,
        )?,
        &[mint_info.clone()],
    )?;

    initialize_mint(
        mint_info,
        signer_info,
        Some(signer_info),
        token_program,
        rent_sysvar,
        args.decimals,
    )?;

    msg!("Transfer Fee Extension: Initialized.");

    Ok(())
}
