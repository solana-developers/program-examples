mod create;
mod increment;

use create::*;
use increment::*;

use program_derived_addresses_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(
        &program_derived_addresses_api::ID,
        program_id,
        instruction_data,
    )?;

    match ix {
        ProgramDerivedAddressesInstruction::Create => process_create(accounts, data)?,
        ProgramDerivedAddressesInstruction::Increment => process_increment(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
