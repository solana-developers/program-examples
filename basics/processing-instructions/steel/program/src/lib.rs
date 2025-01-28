mod go_to_the_park;

use go_to_the_park::*;
use processing_instructions_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing instruction");

    let (ix, data) = parse_instruction(
        &processing_instructions_api::ID,
        program_id,
        instruction_data,
    )?;

    match ix {
        ProcessingInstructionsInstruction::GoToThePark => process_go_to_the_park(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
