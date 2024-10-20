use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instructions;
use crate::state::Favorites;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Ok(address_info) = Favorites::try_from_slice(instruction_data) {
        return instructions::set_favorites::set_favorites(
            program_id,
            accounts,
            address_info.number,
            address_info.color,
            address_info.hobbies,
        );
    };

    Err(ProgramError::InvalidInstructionData)
}
