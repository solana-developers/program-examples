use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey,
};
use borsh::BorshDeserialize;
use crate::instruction::InstructionData;


// For processing the instructions


pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let instruction_data_object = InstructionData::try_from_slice(&instruction_data)?;

    msg!("Welcome to the park, {}!", instruction_data_object.name);
    if instruction_data_object.height > 5 {
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };

    Ok(())
}