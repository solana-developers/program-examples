mod create_address_info;

use account_data_api::prelude::*;
use create_address_info::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&account_data_api::ID, program_id, data)?;

    match ix {
        AddressInfoInstruction::CreateAddressInfo => process_create_address_info(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
