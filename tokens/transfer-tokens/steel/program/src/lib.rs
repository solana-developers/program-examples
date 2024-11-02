mod borsh_instruction;
mod instructions;

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
        SteelInstruction::Create => Create::process(accounts, data)?,
        SteelInstruction::MintNft => MintNft::process(accounts)?,
        SteelInstruction::MintSpl => MintSpl::process(accounts, data)?,
        SteelInstruction::TransferTokens => TransferTokens::process(accounts, data)?,
    }

    Ok(())
}
