mod check_accounts;

use check_accounts::*;
        
use checking_accounts_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&checking_accounts_api::ID, program_id, data)?;

    match ix {
        ValidationInstruction::CheckAccountsArgs => process_check_accounts(accounts, data)?
    }

    Ok(())
}

entrypoint!(process_instruction);
