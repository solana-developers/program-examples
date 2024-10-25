mod create_amm;

use create_amm::*;

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
    }

    Ok(())
}

entrypoint!(process_instruction);
