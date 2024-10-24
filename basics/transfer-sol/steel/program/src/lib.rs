mod transfer_with_cpi;
mod transfer_with_program;

use transfer_with_cpi::*;
use transfer_with_program::*;

use steel::*;
use steel_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::TransferWithProgram => process_transfer_with_program(accounts, data)?,
        SteelInstruction::TransferWithCPI => process_transfer_with_cpi(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
