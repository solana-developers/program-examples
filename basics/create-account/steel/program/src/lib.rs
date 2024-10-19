mod initialize;
use initialize::*;
        
use create_account_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&create_account_api::ID, program_id, data)?;

    match ix {
        CreateAccountInstruction::InitializeNewAccount => process_initialize(accounts)?
    }
    Ok(())
}

entrypoint!(process_instruction);
