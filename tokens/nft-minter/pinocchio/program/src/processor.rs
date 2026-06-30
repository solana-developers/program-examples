use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};
use pinocchio_log::log;

use crate::instructions::{create_token, mint_to};

/// Dispatches an instruction based on its leading discriminator byte.
///
/// The discriminator is the Borsh enum variant index, matching the `native`
/// example's `NftMinterInstruction`:
///   - `0` -> Create (args: `[title: string, symbol: string, uri: string]`)
///   - `1` -> Mint   (no args)
pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, args) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match *discriminator {
        0 => {
            log!("Instruction: Create");
            create_token(accounts, args)
        }
        1 => {
            log!("Instruction: Mint");
            mint_to(accounts)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
