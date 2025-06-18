use solana_program::{msg, program_pack::Pack};
use spl_token_2022::{
    extension::{transfer_fee::instruction::initialize_transfer_fee_config, ExtensionType},
    pod::PodMint,
    state::{Account, Mint},
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
    msg!(&space.to_string());

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
    solana_program::program::invoke(
        &initialize_transfer_fee_config(
            token_program.key,
            mint_info.key,
            Some(signer_info.key),
            Some(signer_info.key),
            transfer_fee_basis_points,
            maximum_fee,
        )?,
        &[
            mint_info.clone(),
            token_program.clone(), // If needed — depending on accounts defined in the program
        ],
    )?;
    //Initialize the Token Mint
    // initialize_mint(mint_info, mint_authority_info, freeze_authority_info, token_program, rent_sysvar, decimals);
    // msg!(&token_program.key.to_string());
    // msg!(&mint_info.key.to_string());
    // msg!(&signer_info.key.to_string());
    // msg!(&args.decimals.to_string());

    solana_program::program::invoke(
        &spl_token_2022::instruction::initialize_mint2(
            token_program.key,
            mint_info.key,
            signer_info.key,
            Some(signer_info.key),
            9,
        )?,
        &[
            mint_info.clone(),
            signer_info.clone(),
            token_program.clone(),
            // rent_sysvar.clone()
        ],
    )?;

    // initialize_mint2(
    //     mint_info,
    //     signer_info,
    //     Some(signer_info),
    //     token_program,
    //     args.decimals,
    // )?;

    // initialize_mint(
    //     mint_info,
    //     signer_info,
    //     Some(signer_info),
    //     token_program,
    //     rent_sysvar,
    //     args.decimals,
    // )?;

    msg!("wani");

    msg!("Space created");
    Ok(())
}
