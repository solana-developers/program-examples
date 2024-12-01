use pda_mint_authority_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&pda_mint_authority_steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Init => Init::process(accounts)?,
        SteelInstruction::CreateToken => CreateToken::process(accounts, data)?,
        SteelInstruction::MintTo => MintTo::process(accounts)?,
    }

    Ok(())
}
