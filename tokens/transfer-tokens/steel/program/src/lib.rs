mod transfer;

use transfer::*;
        
use transfer_sol_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&transfer_sol_api::ID, program_id, data)?;

    match ix {
        TransferInstruction::TransferSolWithCpi => process_transfer_sol_with_cpi(accounts, data)?,
        TransferInstruction::TransferSolWithProgram => process_transfer_sol_with_program(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
