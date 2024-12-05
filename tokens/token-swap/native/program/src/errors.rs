use solana_program::program_error::ProgramError;

pub enum AmmError {
    InvalidFee,
    InvalidMint,
    InvalidAuthority,
    DepositTooSmall,
}

impl From<AmmError> for ProgramError {
    fn from(e: AmmError) -> ProgramError {
        ProgramError::Custom(e as u32)
    }
}
