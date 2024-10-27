mod add;
mod initialize;

use add::*;
use initialize::*;
        
use lever_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&lever_api::ID, program_id, data)?;

    match ix {
        LeverInstruction::Initialize => process_initialize(accounts, data)?,
        LeverInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
