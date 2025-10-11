use crate::instructions::{
    create_counter::process_create_counter, increment_counter::process_increment_counter,
    CounterInstructions,
};
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

pub fn process_instructions(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {
    let (disc, _) = ix_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match CounterInstructions::try_from(disc)? {
        CounterInstructions::CreateCounter => process_create_counter(program_id, accounts)?,
        CounterInstructions::Increment => process_increment_counter(program_id, accounts)?,
    }
    Ok(())
}
