mod increment;
mod initialize;

use increment::*;
use initialize::*;

use counter_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&counter_api::ID, program_id, data)?;

    match ix {
        CounterInstruction::Initialize => process_initialize(accounts, data)?,
        CounterInstruction::Increment => process_increment(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
