use realloc_api::prelude::*;
use steel::*;

mod initialize;
mod update;

use initialize::process_initialize;
use update::process_update;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<ReallocInstruction>(
        &realloc_api::ID,
        program_id,
        data
    )?;

    match ix {
        ReallocInstruction::Initialize => process_initialize(accounts, data)?,
        ReallocInstruction::Update => process_update(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);