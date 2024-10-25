mod hello;

use hello::*;
        
use hello_solana_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, _data) = parse_instruction(&hello_solana_api::ID, program_id, data)?;

    match ix {
        HelloSolanaInstruction::Hello => process_hello(accounts)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
