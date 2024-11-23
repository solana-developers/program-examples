mod token;

use steel::*;
use steel_api::prelude::*;
use token::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Create_Token => process_create_token(accounts, name, symbol)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
