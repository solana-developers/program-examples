use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::{interest_bearing_mint, ExtensionType},
    state::Mint,
};
use steel::*;
use steel_api::prelude::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = Initialize::try_from_bytes(&data)?;
    let rate = i16::from_le_bytes(args.rate);

    // Load accounts.
    let [signer_info, mint_info, system_program, token_program, rent_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    mint_info.is_signer()?.is_empty()?.is_writable()?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token_2022::ID)?;
    rent_sysvar.is_sysvar(&sysvar::rent::ID)?;

    //Calculate space for mint and extension data
    let space =
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::InterestBearingConfig])?;
    //Create account for the mint and allocate space
    create_account(
        signer_info,
        mint_info,
        system_program,
        space,
        token_program.key,
    )?;

    //Initialize the Interest Bearing Mint Extension
    //This instruction must come before the instruction to initialize mint data
    invoke(
        &interest_bearing_mint::instruction::initialize(
            token_program.key,
            mint_info.key,
            Some(*signer_info.key),
            rate,
        )?,
        &[mint_info.clone(), signer_info.clone()],
    )?;

    initialize_mint(
        mint_info,
        signer_info,
        Some(signer_info),
        token_program,
        rent_sysvar,
        6, //you can configure it to be passed as instruction data
    )?;

    msg!("Interest Bearing Mint Extension: Initialized!");

    Ok(())
}
