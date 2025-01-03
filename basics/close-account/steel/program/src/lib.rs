mod close_user;
mod create_user;

use close_user::*;
use create_user::*;

use close_account_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&close_account_api::ID, program_id, data)?;

    match ix {
        CloseAccountInstruction::CreateUser => process_create_user(accounts, data)?,
        CloseAccountInstruction::CloseUser => process_close_user(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
