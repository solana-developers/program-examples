use steel::*;

/// A [Result] type representing `Result<T, CloseAccountError>`
pub type CloseAccountResult<T> = Result<T, CloseAccountError>;

/// Error handling enum for this create
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum CloseAccountError {
    /// A name can only be 64 bytes in length when converted to bytes
    #[error("Invalid Name Length. The maximum length of the string is 64 bytes.")]
    MaxNameLengthExceeded = 0,
    /// Only UTF-8 String types are supported
    #[error("Only UTF-8 String encoding is supported")]
    OnlyUtf8IsSupported = 1,
}

error!(CloseAccountError);
