use pinocchio::error::ProgramError;

mod create;
mod mint;

pub use create::*;
pub use mint::*;

/// Size (in bytes) of an SPL Token mint account.
pub const MINT_SIZE: usize = 82;

/// Decimals for the minted token (the default SPL Token standard).
pub const TOKEN_DECIMALS: u8 = 9;

/// The Metaplex Token Metadata program ID
/// (`metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`).
pub const TOKEN_METADATA_PROGRAM_ID: pinocchio::Address =
    pinocchio::Address::from_str_const("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

/// Borsh-encoded arguments for the `Create` instruction.
///
/// Field order matches the `native` example's `CreateTokenArgs`.
pub struct CreateTokenArgs<'a> {
    pub name: &'a [u8],
    pub symbol: &'a [u8],
    pub uri: &'a [u8],
}

impl<'a> CreateTokenArgs<'a> {
    /// Parses the instruction data: three Borsh strings (title, symbol, uri).
    pub fn parse(data: &'a [u8]) -> Result<Self, ProgramError> {
        let mut offset = 0;
        let name = read_borsh_string(data, &mut offset)?;
        let symbol = read_borsh_string(data, &mut offset)?;
        let uri = read_borsh_string(data, &mut offset)?;
        Ok(Self { name, symbol, uri })
    }
}

/// Reads a little-endian `u64` from the start of `data`.
pub(crate) fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
    let bytes: [u8; 8] = data
        .get(..8)
        .ok_or(ProgramError::InvalidInstructionData)?
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    Ok(u64::from_le_bytes(bytes))
}

/// Reads a Borsh `string` (a 4-byte little-endian length prefix followed by that
/// many UTF-8 bytes) starting at `*offset`, advancing `offset` past it.
fn read_borsh_string<'a>(data: &'a [u8], offset: &mut usize) -> Result<&'a [u8], ProgramError> {
    let len_bytes: [u8; 4] = data
        .get(*offset..*offset + 4)
        .ok_or(ProgramError::InvalidInstructionData)?
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let len = u32::from_le_bytes(len_bytes) as usize;
    *offset += 4;

    let bytes = data
        .get(*offset..*offset + len)
        .ok_or(ProgramError::InvalidInstructionData)?;
    *offset += len;
    Ok(bytes)
}
