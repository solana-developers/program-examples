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
    let (ix, data) = parse_instruction(&crate::ID, program_id, data)?;

    match ix {
        EscrowInstruction::MakeOffer => MakeOffer::process(accounts, data)?,
        EscrowInstruction::TakeOffer => TakeOffer::process(accounts)?,
    }

    Ok(())
}
