use pda_rent_payer_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&pda_rent_payer_steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::InitRentVault => InitRentVault::process(accounts, data)?,
        SteelInstruction::CreateNewAccount => CreateNewAccount::process(accounts)?,
    }

    Ok(())
}
