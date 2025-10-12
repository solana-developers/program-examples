use crate::instructions::{cpi_transfer::*, TransferSolInstructions};
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {
    let (disc, ix_args) = ix_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match TransferSolInstructions::try_from(disc)? {
        TransferSolInstructions::CpiTransfer => {
            process_cpi_transfer(program_id, accounts, ix_args)?
        }
    }

    Ok(())
}
