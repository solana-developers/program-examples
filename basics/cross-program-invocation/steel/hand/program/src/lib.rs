mod initialize;
mod set_power_status;

use initialize::*;
use set_power_status::*;

use hand_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&hand_api::ID, program_id, data)?;

    match ix {
        HandInstruction::Initialize => process_initialize(accounts, data)?,
        HandInstruction::SetPowerStatus => process_set_power_status(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
