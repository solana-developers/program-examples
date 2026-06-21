use pinocchio::error::ProgramError;

mod create;
mod mint;
mod transfer;

pub use create::*;
pub use mint::*;
pub use transfer::*;

/// Size (in bytes) of an SPL Token mint account.
pub const MINT_SIZE: usize = 82;

/// Reads a little-endian `u64` from the start of `data`.
pub(crate) fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
    let bytes: [u8; 8] = data
        .get(..8)
        .ok_or(ProgramError::InvalidInstructionData)?
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    Ok(u64::from_le_bytes(bytes))
}
