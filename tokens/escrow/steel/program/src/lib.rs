mod make_offer;
mod refund;
mod take_offer;

use escrow_api::prelude::*;
use make_offer::*;
use refund::*;
use steel::*;
use take_offer::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&escrow_api::ID, program_id, data)?;

    match ix {
        AccountInstruction::MakeOffer => process_make_offer(accounts, data)?,
        AccountInstruction::TakeOffer => process_take_offer(accounts, data)?,
        AccountInstruction::Refund => process_refund(accounts)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
