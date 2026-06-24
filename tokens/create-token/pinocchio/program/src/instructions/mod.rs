use pinocchio::error::ProgramError;

mod create_token;

pub use create_token::*;

/// Size (in bytes) of an SPL Token mint account.
pub const MINT_SIZE: usize = 82;

/// The Metaplex Token Metadata program ID
/// (`metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`).
pub const TOKEN_METADATA_PROGRAM_ID: pinocchio::Address =
    pinocchio::Address::from_str_const("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

/// Borsh-encoded arguments for the create-token instruction.
///
/// Field order matches the `native` example's `CreateTokenArgs` so the two
/// options share an identical wire format.
pub struct CreateTokenArgs<'a> {
    pub name: &'a [u8],
    pub symbol: &'a [u8],
    pub uri: &'a [u8],
    pub decimals: u8,
}

impl<'a> CreateTokenArgs<'a> {
    /// Parses the instruction data: three Borsh strings followed by a `u8`.
    pub fn parse(data: &'a [u8]) -> Result<Self, ProgramError> {
        let mut offset = 0;
        let name = read_borsh_string(data, &mut offset)?;
        let symbol = read_borsh_string(data, &mut offset)?;
        let uri = read_borsh_string(data, &mut offset)?;
        let decimals = *data
            .get(offset)
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(Self {
            name,
            symbol,
            uri,
            decimals,
        })
    }
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
