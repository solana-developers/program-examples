use escrow_steel_api::prelude::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&escrow_steel_api::ID, program_id, data)?;

    match ix {
        EscrowInstruction::MakeOffer => MakeOffer::process(accounts, data)?,
        EscrowInstruction::TakeOffer => TakeOffer::process(accounts)?,
    }

    Ok(())
}
