use steel::*;
use thiserror::Error;

/// Custom error types for the address info program
#[repr(u32)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum AddressInfoError {
    #[error("Required account is missing")]
    MissingRequiredAccount = 0,

    #[error("Account owner is invalid")]
    InvalidAccountOwner = 1,

    #[error("Invalid instruction data")]
    InvalidInstructionData = 2,
    
    #[error("Address info account already exists")]
    AddressInfoAccountAlreadyExists = 3,
}

error!(AddressInfoError);