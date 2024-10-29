mod create_system_account;
use create_system_account::*;
        
use rent_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&rent_api::ID, program_id, data)?;

    match ix {
        RentInstruction::CreateSystemAccountArgs => process_create_system_account(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
