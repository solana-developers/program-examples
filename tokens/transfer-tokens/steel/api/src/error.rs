use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum SteelError {
    #[error("Failed to parse string from bytes")]
    ParseError = 0,
}

error!(SteelError);
