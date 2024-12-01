use create_account_steel_api::*;
use prelude::{CreateAccount, SteelInstruction};
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, _data) = parse_instruction(&create_account_steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::CreateAccount => CreateAccount::process_instruction(accounts),
    }
}
