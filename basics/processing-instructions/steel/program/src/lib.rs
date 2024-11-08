use processing_instructions_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // get the first bytes from the instruction data as the instruction discriminator
    //
    let (ix, data) = parse_instruction(
        &processing_instructions_steel_api::ID,
        program_id,
        instruction_data,
    )?;

    // match the discriminator
    //
    match ix {
        // process the rest of the data
        //
        ParkInstruction::Park => Park::process_instruction_data(data),
    }
}
