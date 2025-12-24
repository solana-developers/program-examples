use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::instructions::{create_pda::*, get_pda::*};
pub use crate::state::Favorites;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, ix_data) = instruction_data.split_first().unwrap();

    match discriminator {
        1 => create_pda(program_id, accounts, ix_data),
        2 => get_pda(program_id, accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }?;

    Ok(())
}
