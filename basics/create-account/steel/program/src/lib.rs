mod create_system_account;

use create_system_account::*;

use create_account_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&create_account_api::ID, program_id, data)?;

    match ix {
        CreateAccountInstruction::CreateSystemAccount => {
            process_create_system_account(accounts, data)?
        }
    }

    Ok(())
}

entrypoint!(process_instruction);
