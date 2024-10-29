mod pull_lever;

use pull_lever::*;
        
use hand_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&hand_api::ID, program_id, data)?;

    match ix {
        HandInstruction::PullLeverArgs => process_pull_lever(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
