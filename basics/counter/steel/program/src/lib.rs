mod add;
mod initialize;

use add::*;
use initialize::*;
        
use steelcounter_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steelcounter_api::ID, program_id, data)?;

    match ix {
        SteelcounterInstruction::Initialize => process_initialize(accounts, data)?,
        SteelcounterInstruction::Add => process_add(accounts)?,
    }
    Ok(())
}

entrypoint!(process_instruction);
