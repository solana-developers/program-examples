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
    // use `crate::ID` as program_id in your program
    // 
    // e.g parse_instruction(&crate::ID, program_id, data)
    // 
    let (ix, data) = parse_instruction(program_id, program_id, data)?;

    match ix {
        EscrowInstruction::MakeOffer => MakeOffer::process(program_id, accounts, data)?,
        EscrowInstruction::TakeOffer => TakeOffer::process(program_id, accounts)?,
    }

    Ok(())
}
