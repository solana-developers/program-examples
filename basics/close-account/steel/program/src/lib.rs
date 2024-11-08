use close_account_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&close_account_steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::CreateUser => CreateUser::process(program_id, accounts, data),
        SteelInstruction::CloseUser => CloseUser::process(program_id, accounts),
    }
}
