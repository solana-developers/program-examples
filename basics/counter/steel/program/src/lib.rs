mod increment;
mod initialize;

use increment::*;
use initialize::*;
        
use steel_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Initialize => process_initialize(accounts, data)?,
        SteelInstruction::Increment => process_increment(accounts)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
