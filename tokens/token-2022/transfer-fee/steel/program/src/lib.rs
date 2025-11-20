mod harvest;
mod initialize;
mod transfer;
mod update_fee;
mod withdraw;

use harvest::*;
use initialize::*;
use transfer::*;
use update_fee::*;
use withdraw::*;

use steel::*;
use steel_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Initialize => process_initialize(accounts, data)?,
        SteelInstruction::Transfer => process_transfer(accounts, data)?,
        SteelInstruction::Harvest => process_harvest(accounts, data)?,
        SteelInstruction::Withdraw => process_withdraw(accounts, data)?,
        SteelInstruction::UpdateFee => process_update_fee(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
