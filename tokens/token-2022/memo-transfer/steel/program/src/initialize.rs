use solana_program::{msg, program::invoke};
use spl_token_2022::{
    extension::{memo_transfer::instruction::enable_required_transfer_memos, ExtensionType},
    instruction::initialize_account3,
    state::Account,
};
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, mint_info, token_account_info, system_program, token_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    msg!("Account loaded");

    //Validation
    signer_info.is_signer()?;
    token_account_info.is_writable()?;
    token_program.is_program(&spl_token_2022::ID)?;
    system_program.is_program(&system_program::ID)?;

    //Calculate space for token and extension data
    let space =
        ExtensionType::try_calculate_account_len::<Account>(&[ExtensionType::MemoTransfer])?;

    //Create new account for token and allocate space
    create_account(
        signer_info,
        token_account_info,
        system_program,
        space,
        token_program.key,
    )?;

    msg!(&token_account_info.key.to_string());

    invoke(
        &initialize_account3(
            token_program.key,
            token_account_info.key,
            mint_info.key,
            signer_info.key,
        )?,
        &[token_account_info.clone(), mint_info.clone()],
    )?;

    //Initialize the memo on transfer extension
    //This instruction must come after the instruction to initialize account
    invoke(
        &enable_required_transfer_memos(
            token_program.key,
            token_account_info.key,
            signer_info.key,
            &[],
        )?,
        &[token_account_info.clone(), signer_info.clone()],
    )?;

    msg!("Memo Transfer Extension: Initialized.");

    Ok(())
}
