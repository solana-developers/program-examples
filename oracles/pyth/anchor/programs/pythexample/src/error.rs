use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("PythError")]
    PythError,
    #[msg("TryToSerializePriceAccount")]
    TryToSerializePriceAccount,
    #[msg("InvalidArgument")]
    InvalidArgument,
}
