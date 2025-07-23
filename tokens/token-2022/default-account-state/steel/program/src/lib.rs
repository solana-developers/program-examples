mod initialize;
mod update_default_state;

use initialize::*;
use update_default_state::*;

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
        SteelInstruction::UpdateDefaultAccountState => {
            process_update_default_state(accounts, data)?
        }
    }

    Ok(())
}

entrypoint!(process_instruction);
