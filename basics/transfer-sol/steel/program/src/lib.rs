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
    let (ix, data) = parse_instruction(&crate::ID, program_id, data)?;

    match ix {
        TransferInstruction::TransferSolWithCpi => TransferSolWithCpi::process(accounts, data),
        TransferInstruction::TransferSolWithProgram => {
            TransferSolWithProgram::process(accounts, data)
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TransferInstruction {
    TransferSolWithCpi = 0,
    TransferSolWithProgram = 1,
}
