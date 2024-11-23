use spl_token_minter_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&spl_token_minter_steel_api::ID, program_id, data)?;
    match ix {
        SteelInstruction::CreateToken => CreateToken::process(accounts, data)?,
        SteelInstruction::MintTo => MintTo::process(accounts, data)?,
    }

    Ok(())
}
