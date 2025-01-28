mod transfer_sol_with_cpi;
mod transfer_sol_with_program;

use transfer_sol_with_cpi::*;
use transfer_sol_with_program::*;
        
use transfer_sol_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&transfer_sol_api::ID, program_id, data)?;

    match ix {
        TransferSolInstruction::TransferSolWithCpi => process_transfer_sol_with_cpi(accounts, data)?,
        TransferSolInstruction::TransferSolWithProgram => process_transfer_sol_with_program(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
