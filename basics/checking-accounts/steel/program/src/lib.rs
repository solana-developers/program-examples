mod initialize;
use initialize::*;
        
use checking_account_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&checking_account_api::ID, program_id, data)?;

    match ix {
        AccountInstruction::InitializeAccount => process_accounts(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
