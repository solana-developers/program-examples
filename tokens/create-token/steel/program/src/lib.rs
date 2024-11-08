use create_token_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&create_token_steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::CreateToken => CreateToken::process(accounts, data),
    }
}
