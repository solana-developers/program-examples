#![no_std]
#![allow(deprecated)]

use pinocchio::{
    account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError,
    pubkey::Pubkey, ProgramResult,
};
use pinocchio_system::instructions::Transfer;

entrypoint!(process_instruction);
nostd_panic_handler!();

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((&CPI_TRANSFER_DISCRIMINATOR, data)) => transfer_sol_with_cpi(accounts, data),
        Some((&PROGRAM_TRANSFER_DISCRIMINATOR, data)) => transfer_sol_with_program(accounts, data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub const CPI_TRANSFER_DISCRIMINATOR: u8 = 0;
pub const PROGRAM_TRANSFER_DISCRIMINATOR: u8 = 1;

fn transfer_sol_with_cpi(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let [payer, recipient, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount_bytes: [u8; 8] = instruction_data[0..8]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let amount = u64::from_le_bytes(amount_bytes);

    Transfer {
        from: payer,
        to: recipient,
        lamports: amount,
    }
    .invoke()?;

    Ok(())
}

fn transfer_sol_with_program(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let [payer, recipient] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount_bytes: [u8; 8] = instruction_data[0..8]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let amount = u64::from_le_bytes(amount_bytes);

    *payer.try_borrow_mut_lamports()? -= amount;
    *recipient.try_borrow_mut_lamports()? += amount;

    Ok(())
}
