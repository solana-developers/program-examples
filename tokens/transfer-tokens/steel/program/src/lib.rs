mod create;
mod mint;
mod transfer;

use create::*;
use mint::*;
use transfer::*;

use steel::*;
use transfer_tokens_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&transfer_tokens_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Mint => process_mint(accounts, data)?,
        SteelInstruction::Create => process_create(accounts, data)?,
        SteelInstruction::Transfer => process_transfer(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
