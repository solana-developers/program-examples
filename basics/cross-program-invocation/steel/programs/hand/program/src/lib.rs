use cross_program_invocation_steel_hand_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(
        &cross_program_invocation_steel_hand_api::ID,
        program_id,
        data,
    )?;

    match ix {
        HandInstruction::PullLever => PullLever::process(accounts, data),
    }
}
