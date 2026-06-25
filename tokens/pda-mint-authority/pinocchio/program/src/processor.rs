use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};
use pinocchio_log::log;

use crate::instructions::{create_token, init, mint_to};

/// Dispatches an instruction based on its leading discriminator byte.
///
/// The discriminator is the Borsh enum variant index, matching the `native`
/// example's `MyInstruction`:
///   - `0` -> Init   (args: `[bump: u8]`)
///   - `1` -> Create (args: `[title: string, symbol: string, uri: string]`)
///   - `2` -> Mint   (no args)
pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, args) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match *discriminator {
        0 => {
            log!("Instruction: Init");
            init(program_id, accounts, args)
        }
        1 => {
            log!("Instruction: Create");
            create_token(program_id, accounts, args)
        }
        2 => {
            log!("Instruction: Mint");
            mint_to(program_id, accounts)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
