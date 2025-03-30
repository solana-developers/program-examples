use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Could not load price account")]
    PythError,
    #[msg("Invalid argument provided")]
    InvalidArgument,
}
