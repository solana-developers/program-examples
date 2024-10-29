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
    // Use crate::ID for program_id instead: 
    // e.g parse_instruction(&crate::ID, program_id, data)
    // using program_id for testing purposes
    //
    let (ix, data) = parse_instruction(program_id, program_id, data)?;

    match ix {
        SteelInstruction::CreateUser => CreateUser::process(program_id, accounts, data),
        SteelInstruction::CloseUser => CloseUser::process(program_id, accounts),
    }
}
