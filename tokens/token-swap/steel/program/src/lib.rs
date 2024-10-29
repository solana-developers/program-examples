mod add;
mod initialize;

use add::*;
use initialize::*;
        
use token_swap_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&token_swap_api::ID, program_id, data)?;

    match ix {
        TokenSwapInstruction::Initialize => process_initialize(accounts, data)?,
        TokenSwapInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
