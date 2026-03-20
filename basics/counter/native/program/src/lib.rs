use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    declare_id,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

mod state;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_discriminant, instruction_data_inner) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match instruction_discriminant {
        0 => {
            msg!("Instruction: Increment");
            process_increment_counter(program_id, accounts, instruction_data_inner)?;
        }
        _ => {
            msg!("Error: unknown instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}

pub fn process_increment_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let counter_account = next_account_info(account_info_iter)?;

    // Never panic in on-chain programs: panics consume all compute units and are hard to debug.
    if !counter_account.is_writable {
        msg!("Counter account must be writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Ownership check prevents unexpected state corruption when a wrong account is passed.
    if counter_account.owner != program_id {
        msg!("Counter account is not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Use a single mutable borrow to avoid runtime borrow conflicts.
    let mut data = counter_account.try_borrow_mut_data()?;
    let mut counter = Counter::try_from_slice(&data)?;

    counter.count = counter
        .count
        .checked_add(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    counter.serialize(&mut &mut data[..])?;

    msg!("Counter state incremented to {:?}", counter.count);
    Ok(())
}
