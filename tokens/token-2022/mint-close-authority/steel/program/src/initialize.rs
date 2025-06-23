use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::{mint_close_authority, ExtensionType},
    instruction::initialize_mint_close_authority,
    state::Mint,
};
use steel::*;

//Here the close authority is the signer info
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
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
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MintCloseAuthority])?;
    msg!(&space.to_string());
    //Create account for the mint and allocate space
    create_account(
        signer_info,
        mint_info,
        system_program,
        space,
        token_program.key,
    )?;

    //Initialize the Mint-Close-Authority Extension
    //This instruction must come before the instruction to initialize mint data
    invoke(
        &initialize_mint_close_authority(token_program.key, mint_info.key, Some(signer_info.key))?,
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

    msg!("Mint Close Authority Extension: Initialized!");

    Ok(())
}
