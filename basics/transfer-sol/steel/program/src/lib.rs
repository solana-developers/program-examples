use solana_program::msg;

mod transfer;

use transfer::*;
        
use transfer_sol_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Instruction data length: {}", data.len());
    msg!("Raw instruction data: {:?}", data);

    // Parse_instruction 
    let (ix, args_data) = parse_instruction::<TransferInstruction>(&transfer_sol_api::ID, program_id, data)?;
    msg!("✅ Parsed instruction type: {:?}", ix);

    match ix {
        TransferInstruction::TransferSolWithCpi => {
            msg!("Processing TransferSolWithCpi");
            process_transfer_sol_with_cpi(accounts, args_data)
        }
        TransferInstruction::TransferSolWithProgram => {
            msg!("Processing TransferSolWithProgram");
            process_transfer_sol_with_program(accounts, args_data)
        }
    }
}

entrypoint!(process_instruction);
