mod add;
mod initialize;

use add::*;
use initialize::*;
        
use transfer_sol_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&transfer_sol_api::ID, program_id, data)?;

    match ix {
        TransferSolInstruction::Initialize => process_initialize(accounts, data)?,
        TransferSolInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
