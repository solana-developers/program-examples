mod create_account;

use create_account::*;
use rent_example_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<RentInstruction>(&rent_example_api::ID, program_id, data)?;

    match ix {
        RentInstruction::CreateSystemAccount => process_create_account(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);