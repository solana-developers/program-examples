mod create;
mod init;
mod mint;

pub use create::*;
pub use init::*;
pub use mint::*;

use pda_mint_authority_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&pda_mint_authority_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Create => process_create(accounts, data)?,
        SteelInstruction::Mint => process_mint(accounts, data)?,
        SteelInstruction::Init => process_init(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
