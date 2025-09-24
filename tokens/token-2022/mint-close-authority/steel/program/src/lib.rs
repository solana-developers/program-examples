mod initialize;
mod mint_close;

use initialize::*;
use mint_close::*;

use steel::*;
use steel_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Initialize => process_initialize(accounts, data)?,
        SteelInstruction::MintCloseAuthority => process_mint_close(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
