use solana_program::program_error::ProgramError;

pub enum AmmError {
    InvalidFee,
}

impl From<AmmError> for ProgramError {
    fn from(e: AmmError) -> ProgramError {
        ProgramError::Custom(e as u32)
    }
}
