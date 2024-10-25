mod initialize;
use initialize::*;
mod close;
use close::*;
        
use close_account_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&close_account_api::ID, program_id, data)?;

    match ix {
        AccountInstruction::InitializeAccount => process_init(accounts, data)?,
        AccountInstruction::CloseAccount => process_close(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
