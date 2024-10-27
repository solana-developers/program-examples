mod initialize;

use create_account_api::prelude::*;
use initialize::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, _data) = parse_instruction(&create_account_api::ID, program_id, data)?;

    match ix {
        CreateAccountInstruction::InitializeAccount => process_initialize(accounts)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
