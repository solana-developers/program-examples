use steel::*;

#[repr(u32)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum RentError {
    #[error("String too long")]
    StringTooLong = 0,
}

error!(RentError);