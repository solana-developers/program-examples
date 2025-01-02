mod create_address_info;

use account_data_api::prelude::*;
use create_address_info::*;
use solana_program::msg;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Validate program ID
    if program_id != &account_data_api::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse and validate instruction data
    let (instruction, instruction_data) =
        parse_instruction::<AddressInfoInstruction>(&account_data_api::ID, program_id, data)?;

    // Route instruction to appropriate processor
    match instruction {
        AddressInfoInstruction::CreateAddressInfo => {
            msg!("Instruction: CreateAddressInfo");
            process_create_address_info(accounts, instruction_data)?
        }
    }

    Ok(())
}

entrypoint!(process_instruction);
