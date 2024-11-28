use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum ReallocError {
    #[error("Invalid string length")]
    InvalidStringLength = 0,
}

error!(ReallocError);