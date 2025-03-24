mod create_token;
use solana_program::msg;

use create_token::*;
        
use steel_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Instruction started");
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        TokenInstruction::CreateToken => process_create_token(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
