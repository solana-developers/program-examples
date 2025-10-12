use crate::instructions::{create_page::*, increment_page_visits::*, CreatePageInstructions};
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {
    let (disc, _) = ix_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match CreatePageInstructions::try_from(disc)? {
        CreatePageInstructions::CreatePage => process_create_page(program_id, accounts)?,
        CreatePageInstructions::IncrementPageVisits => {
            process_increament_page_visits(program_id, accounts)?
        }
    }

    Ok(())
}
