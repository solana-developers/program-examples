mod create_amm;
mod create_pool;
use create_amm::*;
use create_pool::*;

use steel::*;
use token_swap_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&token_swap_api::ID, program_id, data)?;

    match ix {
        TokenSwapInstruction::CreateAmm => process_create_amm(accounts, data)?,
        TokenSwapInstruction::CreatePool => process_create_pool(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
