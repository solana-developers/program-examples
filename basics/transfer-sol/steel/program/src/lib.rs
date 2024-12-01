mod with_cpi;
mod with_program;

use with_cpi::*;
use with_program::*;

use steel::*;
use transfer_sol_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&transfer_sol_api::ID, program_id, data)?;

    match ix {
        TransferSolInstruction::WithCpi => process_with_cpi(accounts, data)?,
        TransferSolInstruction::WithProgram => proces_with_program(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
