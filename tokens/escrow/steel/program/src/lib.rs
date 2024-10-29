mod instructions;
mod state;

use instructions::*;
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(program_id, program_id, data)?;

    match ix {
        SteelInstruction::MakeOffer => MakeOffer::process(program_id, accounts, data)?,
        SteelInstruction::TakeOffer => TakeOffer::process(program_id, accounts)?,
    }

    Ok(())
}
