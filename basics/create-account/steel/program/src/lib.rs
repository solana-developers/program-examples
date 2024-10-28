mod initialize;

use create_account_api::prelude::*;
use initialize::*;
use steel::*;

/// Process the input before sending it over
/// to our main program to use.
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

// Declare entrypoint for our program
entrypoint!(process_instruction);
