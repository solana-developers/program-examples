use realloc_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&realloc_steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::CreateAddressInfo => CreateAddressInfo::process(accounts, data),
        SteelInstruction::ExtendAddressInfo => ExtendAddressInfo::process(accounts, data),
        SteelInstruction::ZeroInit => ZeroInit::process(accounts, data),
    }
}
