use steel::*;

#[repr(u32)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum TransferError {
    #[error("Invalid amount")]
    InvalidAmount = 0,
}

error!(TransferError);