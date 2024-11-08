use steel::*;
use transfer_tokens_steel_api::prelude::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&transfer_tokens_steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Create => Create::process(accounts, data)?,
        SteelInstruction::MintNft => MintNft::process(accounts)?,
        SteelInstruction::MintSpl => MintSpl::process(accounts, data)?,
        SteelInstruction::TransferTokens => TransferTokens::process(accounts, data)?,
    }

    Ok(())
}
