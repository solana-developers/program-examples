mod instructions;
mod state;

use instructions::{CloseUser, CreateUser, SteelInstruction};
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&crate::ID, program_id, data)?;

    match ix {
        SteelInstruction::CreateUser => CreateUser::process(program_id, accounts, data),
        SteelInstruction::CloseUser => CloseUser::process(program_id, accounts),
    }
}
