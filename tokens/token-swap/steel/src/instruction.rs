// src/instruction.rs

use solana_program::program_error::ProgramError;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum SwapInstruction {
    /// Swaps tokens between two accounts.
    Swap { amount: u64 },
}

impl SwapInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        SwapInstruction::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}
