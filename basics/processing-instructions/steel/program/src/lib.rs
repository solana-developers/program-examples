use processing_instructions_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing instruction");

    let (_tag, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let instruction = GoToThePark::try_from_bytes(instruction_data)?;

    msg!("Welcome to the park, {}", instruction.data.name());

    if instruction.data.height() > 5 {
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };
    Ok(())
}

entrypoint!(process_instruction);
