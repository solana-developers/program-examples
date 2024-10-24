mod check_accounts;

use check_accounts::*;

use steel::*;
use steel_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::CheckAccounts => process_check_accounts(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
