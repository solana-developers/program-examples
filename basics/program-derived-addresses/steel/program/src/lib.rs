mod instructions;
mod state;

use instructions::*;
use state::*;

use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Parse data for instruction discriminator.
    let (ix, _data) = parse_instruction(&crate::ID, program_id, data)?;

    match ix {
        SteelInstruction::CreatePageVisits => CreatePageVisits::process(accounts),
        SteelInstruction::IncrementPageVisits => IncrementPageVisits::process(accounts),
    }
}
