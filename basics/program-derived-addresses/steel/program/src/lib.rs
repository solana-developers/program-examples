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
    let (tag, _) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let ix = SteelInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))?;

    match ix {
        SteelInstruction::CreatePageVisits => CreatePageVisits::process(program_id, accounts),
        SteelInstruction::IncrementPageVisits => IncrementPageVisits::process(program_id, accounts),
    }
}
