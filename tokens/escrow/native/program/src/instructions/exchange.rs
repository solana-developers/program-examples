use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

use crate::{error::EscrowError, state::Escrow};

pub fn exchange(
    accounts: &[AccountInfo],
    amount_expected_by_taker: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let taker = next_account_info(account_info_iter)?;

    if !taker.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let takers_sending_token_account = next_account_info(account_info_iter)?;
    let takers_token_to_receive_account = next_account_info(account_info_iter)?;
    let pdas_temp_token_account = next_account_info(account_info_iter)?;
    let pdas_temp_token_account_info =
        TokenAccount::unpack(&pdas_temp_token_account.try_borrow_data()?)?;
    let (pda, bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

    if amount_expected_by_taker != pdas_temp_token_account_info.amount {
        return Err(EscrowError::ExpectedAmountMismatch.into());
    }

    let initializers_main_account = next_account_info(account_info_iter)?;
    let initializers_token_to_receive_account = next_account_info(account_info_iter)?;
    let escrow_account = next_account_info(account_info_iter)?;

    let escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;

    if escrow_info.temp_token_account_pubkey != *pdas_temp_token_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_info.initializer_pubkey != *initializers_main_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_info.initializer_token_to_receive_account_pubkey != *initializers_token_to_receive_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    let token_program = next_account_info(account_info_iter)?;

    let transfer_to_initializer_ix = spl_token::instruction::transfer(
        token_program.key,
        takers_sending_token_account.key,
        initializers_token_to_receive_account.key,
        taker.key,
        &[&taker.key],
        escrow_info.expected_amount,
    )?;
    msg!("Calling the token program to transfer tokens to the escrow's initializer...");
    solana_program::program::invoke(
        &transfer_to_initializer_ix,
        &[
            takers_sending_token_account.clone(),
            initializers_token_to_receive_account.clone(),
            taker.clone(),
            token_program.clone(),
        ],
    )?;

    let pda_account = next_account_info(account_info_iter)?;

    let transfer_to_taker_ix = spl_token::instruction::transfer(
        token_program.key,
        pdas_temp_token_account.key,
        takers_token_to_receive_account.key,
        &pda,
        &[&pda],
        pdas_temp_token_account_info.amount,
    )?;
    msg!("Calling the token program to transfer tokens to the taker...");
    solana_program::program::invoke_signed(
        &transfer_to_taker_ix,
        &[
            pdas_temp_token_account.clone(),
            takers_token_to_receive_account.clone(),
            pda_account.clone(),
            token_program.clone(),
        ],
        &[&[&b"escrow"[..], &[bump_seed]]],
    )?;

    let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
        token_program.key,
        pdas_temp_token_account.key,
        initializers_main_account.key,
        &pda,
        &[&pda]
    )?;
    msg!("Calling the token program to close pda's temp account...");
    solana_program::program::invoke_signed(
        &close_pdas_temp_acc_ix,
        &[
            pdas_temp_token_account.clone(),
            initializers_main_account.clone(),
            pda_account.clone(),
            token_program.clone(),
        ],
        &[&[&b"escrow"[..], &[bump_seed]]],
    )?;

    msg!("Closing the escrow account...");
    **initializers_main_account.lamports.borrow_mut() = initializers_main_account.lamports()
        .checked_add(escrow_account.lamports())
        .ok_or(EscrowError::AmountOverflow)?;
    **escrow_account.lamports.borrow_mut() = 0;
    *escrow_account.try_borrow_mut_data()? = &mut [];

    Ok(())
}