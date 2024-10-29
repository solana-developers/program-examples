mod with_cpi;
mod with_program;

use steel::*;
use with_cpi::*;
use with_program::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Use crate::ID for program_id instead: 
    // e.g parse_instruction(&crate::ID, program_id, data)
    // using program_id for testing purposes
    //
    let (ix, data) = parse_instruction(program_id, program_id, data)?;

    match ix {
        SteelInstruction::TransferSolWithCpi => TransferSolWithCpi::process(accounts, data),
        SteelInstruction::TransferSolWithProgram => {
            TransferSolWithProgram::process(program_id, accounts, data)
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    TransferSolWithCpi = 0,
    TransferSolWithProgram = 1,
}
