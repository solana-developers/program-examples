mod set_favorites;

pub use set_favorites::*;

use steel::*;
use steel_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::SetFavorites => process_set_favorites(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
