mod initialize;
mod update_fee;

use initialize::*;
use update_fee::*;

use steel::*;
use steel_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Initialize => process_initialize(accounts, data)?,
        SteelInstruction::UpdateRate => process_update_rate(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
