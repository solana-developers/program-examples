mod create_system_account;
        
use steel_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::CreateSystemAccount => process_initialize(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
