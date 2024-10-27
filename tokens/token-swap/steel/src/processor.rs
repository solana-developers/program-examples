// src/processor.rs

use crate::instruction::SwapInstruction;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};

pub fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SwapInstruction::unpack(instruction_data)?;

    let accounts_iter = &mut accounts.iter();
    let source_account = next_account_info(accounts_iter)?;
    let destination_account = next_account_info(accounts_iter)?;

    match instruction {
        SwapInstruction::Swap { amount } => {
            let mut source_balance = source_account.lamports.borrow_mut();
            let mut destination_balance = destination_account.lamports.borrow_mut();

            if *source_balance < amount {
                return Err(ProgramError::InsufficientFunds);
            }

            *source_balance -= amount;
            *destination_balance += amount;
        }
    }

    Ok(())
}
