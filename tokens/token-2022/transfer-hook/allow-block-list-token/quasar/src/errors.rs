use quasar_lang::prelude::ProgramError;

/// Custom error codes for the allow/block list program.
/// Encoded as ProgramError::Custom(N).

pub const ERROR_INVALID_METADATA: u32 = 6000;
pub const ERROR_WALLET_NOT_ALLOWED: u32 = 6001;
pub const ERROR_AMOUNT_NOT_ALLOWED: u32 = 6002;
pub const ERROR_WALLET_BLOCKED: u32 = 6003;
pub const ERROR_UNAUTHORIZED: u32 = 6004;

pub fn invalid_metadata() -> ProgramError {
    ProgramError::Custom(ERROR_INVALID_METADATA)
}

pub fn wallet_not_allowed() -> ProgramError {
    ProgramError::Custom(ERROR_WALLET_NOT_ALLOWED)
}

pub fn amount_not_allowed() -> ProgramError {
    ProgramError::Custom(ERROR_AMOUNT_NOT_ALLOWED)
}

pub fn wallet_blocked() -> ProgramError {
    ProgramError::Custom(ERROR_WALLET_BLOCKED)
}

pub fn unauthorized() -> ProgramError {
    ProgramError::Custom(ERROR_UNAUTHORIZED)
}
