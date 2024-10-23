#![allow(clippy::result_large_err)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
    system_instruction,
    rent::Rent,
    invoke_signed,
};
use spl_token::{self, state::Mint};

declare_id!("3of89Z9jwek9zrFgpCWc9jZvQvitpVMxpZNsrAD2vQUD");

/// Entry point for the SPL Token Minter program
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => create_token(accounts, instruction_data),
        1 => mint_token(accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Function to create a new SPL Token Mint
fn create_token(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Ensure that the accounts provided are correct
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?; // Account paying for the mint creation
    let mint_account = next_account_info(accounts_iter)?; // The new mint account
    let rent_account = next_account_info(accounts_iter)?; // Rent account

    // Ensure this account is a writable account
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Create a new token mint
    let mint = Mint {
        mint_authority: Some(payer.key.clone()),
        supply: 0,
        decimals: 9, // Standard for SPL tokens
        is_initialized: true,
        freeze_authority: None,
    };

    let rent = &Rent::from_account_info(rent_account)?;
    let required_lamports = rent.minimum_balance(Mint::LEN);
    if **mint_account.lamports.borrow() < required_lamports {
        // Create account if it doesn't have enough lamports
        let create_account_ix = system_instruction::create_account(
            payer.key,
            mint_account.key,
            required_lamports,
            Mint::LEN as u64,
            &spl_token::id(),
        );
        msg!("Calling create_account");
        invoke_signed(
            &create_account_ix,
            &[payer.clone(), mint_account.clone(), rent_account.clone()],
            &[],
        )?;
    }

    // Initialize the mint
    msg!("Initializing mint");
    spl_token::initialize_mint(
        &spl_token::id(),
        mint_account.key,
        payer.key,
        None,
        9, // Decimal places
    )?;

    msg!("Token mint created successfully.");
    Ok(())
}

/// Function to mint tokens
fn mint_token(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Ensure that the accounts provided are correct
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?; // The mint account
    let destination_account = next_account_info(accounts_iter)?; // Where tokens are minted to
    let mint_authority = next_account_info(accounts_iter)?; // The account that has authority to mint

    // Extract the amount of tokens to mint
    let amount = instruction_data[1] as u64; // Example to get token amount

    // Mint the tokens
    msg!("Minting {} tokens to {}", amount, destination_account.key);
    spl_token::mint_to(
        &spl_token::id(),
        mint_account.key,
        destination_account.key,
        mint_authority.key,
        &[],
        amount,
    )?;

    msg!("Minted {} tokens successfully.", amount);
    Ok(())
}
