mod add;
mod initialize;

use add::*;
use initialize::*;
        
use my_steel_project_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&my_steel_project_api::ID, program_id, data)?;

    match ix {
        MySteelProjectInstruction::Initialize => process_initialize(accounts, data)?,
        MySteelProjectInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
