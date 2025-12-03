use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::{
        default_account_state::instruction::initialize_default_account_state, ExtensionType,
    },
    state::{AccountState, Mint},
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
    mint_info.is_empty()?.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;

    //Calculate space for token and extension data
    let space =
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::DefaultAccountState])?;

    //Create new account for token and allocate space
    create_account(
        signer_info,
        mint_info,
        system_program,
        space,
        token_program.key,
    )?;

    //Initialize the default account state extension
    //This instruction must come after the instruction to initialize account
    invoke(
        &initialize_default_account_state(token_program.key, mint_info.key, &AccountState::Frozen)?,
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

    msg!("Default Account State Extension: Initialized.");

    Ok(())
}
