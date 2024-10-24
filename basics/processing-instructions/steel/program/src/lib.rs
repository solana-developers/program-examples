use processing_instructions_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing instruction");

    let instruction = GoToTheParkData::try_from_bytes(instruction_data)?;

    msg!("Welcome to the park, {}!", instruction.name());

    if instruction.height() > 5 {
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };
    Ok(())
}

entrypoint!(process_instruction);
