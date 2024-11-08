use checking_accounts_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, _data) = parse_instruction(&checking_accounts_steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::CheckingAccounts => CheckingAccounts::process(accounts),
    }
}
