mod borsh_instruction;
mod instructions;
mod state;

use instructions::*;
use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&crate::ID, program_id, data)?;

    match ix {
        SteelInstruction::Init => Init::process(program_id, accounts)?,
        SteelInstruction::CreateToken => CreateToken::process(program_id, accounts, data)?,
        SteelInstruction::MintTo => MintTo::process(program_id, accounts)?,
    }

    Ok(())
}
