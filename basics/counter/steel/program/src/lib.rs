mod increment;
mod initialize;

use increment::*;
use initialize::*;

use steel::*;
use steel_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, _data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Initialize => process_initialize(accounts)?,
        SteelInstruction::Increment => process_increment(accounts)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
