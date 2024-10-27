mod make_offer;
mod take_offer;

use make_offer::*;
use take_offer::*;

use escrow_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&escrow_api::ID, program_id, data)?;

    match ix {
        EscrowInstruction::MakeOffer => process_make_offer(accounts, data)?,
        EscrowInstruction::TakeOffer => process_take_offer(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
