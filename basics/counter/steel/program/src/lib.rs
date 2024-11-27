use counter_solana_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, _data) = parse_instruction(&counter_solana_steel_api::ID, program_id, data)?;

    match ix {
        CounterInstruction::Initialize => Initialize::process(accounts),
        CounterInstruction::Increment => Increment::process(accounts),
    }
}
