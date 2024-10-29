mod initialize;
mod update;

use update::*;
use initialize::*;
        
use realloc_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&realloc_api::ID, program_id, data)?;

    match ix {
        ReallocInstruction::Initialize => process_initialize(accounts, data)?,
        ReallocInstruction::Update => process_update(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
