use steel::*;

#[repr(u32)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum ReallocError {
    #[error("String too long")]
    StringTooLong = 0,
    #[error("Insufficient rent for realloc")]
    InsufficientRentForRealloc = 1,
}

error!(ReallocError);
