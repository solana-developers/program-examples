mod instructions;
mod state;

use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo, 
        declare_id,
        entrypoint, 
        entrypoint::ProgramResult, 
        pubkey::Pubkey,
    },
};
use crate::instructions::*;

declare_id!("8avNGHVXDwsELJaWMSoUZ44CirQd4zyU9Ez4ZmP4jNjZ");
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let instruction = CarRentalServiceInstruction::try_from_slice(instruction_data)?;
    match instruction {
        CarRentalServiceInstruction::AddCar(car) => add_car(program_id, accounts, car),
        CarRentalServiceInstruction::BookRental(order) => book_rental(program_id, accounts, order),
        CarRentalServiceInstruction::PickUpCar => pick_up_car(program_id, accounts),
        CarRentalServiceInstruction::ReturnCar => return_car(program_id, accounts),
    }
}