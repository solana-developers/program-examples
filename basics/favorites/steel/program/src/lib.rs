use favorites_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&favorites_steel_api::ID, program_id, data)?;

    match ix {
        FavoritesInstruction::SetFavorites => SetFavorites::process(accounts, data),
    }
}
