use steel::*;

#[repr(u32)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum ValidationError {
    #[error("Invalid account owner")]
    InvalidOwner = 0,
}

error!(ValidationError);
