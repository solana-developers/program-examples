mod hello;

use hello::*;

use hello_solana_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&hello_solana_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::HelloSolana => process_hello(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
