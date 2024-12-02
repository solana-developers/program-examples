use cross_program_invocation_steel_lever_api::prelude::*;
use steel::*;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(
        &cross_program_invocation_steel_lever_api::ID,
        program_id,
        data,
    )?;

    match ix {
        LeverInstruction::InitializeLever => InitializeLever::process(accounts, data),
        LeverInstruction::SetPowerStatus => SetPowerStatus::process(accounts, data),
    }
}
