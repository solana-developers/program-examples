mod switch_power;
mod initialize;

use switch_power::*;
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
        LeverInstruction::InitializeArgs => process_initialize(accounts, data)?,
        LeverInstruction::SwitchPowerArgs => process_switch_power(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
