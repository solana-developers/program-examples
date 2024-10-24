use close_account_api::prelude::*;
use steel::*;

mod create_user;
pub(crate) use create_user::*;

mod close_user;
pub(crate) use close_user::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&close_account_api::ID, program_id, data)?;

    match ix {
        MyInstruction::CreateAccount => create_user(accounts, data)?,
        MyInstruction::CloseAccount => close_user(accounts)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
