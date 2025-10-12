use pinocchio::program_error::ProgramError;

pub mod cpi_transfer;

#[repr(u8)]
pub enum TransferSolInstructions {
    CpiTransfer,
}

impl TryFrom<&u8> for TransferSolInstructions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(TransferSolInstructions::CpiTransfer),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
