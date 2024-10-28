use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum MySteelProjectError {
    #[error("This is a dummy error")]
    Dummy = 0,
}

error!(MySteelProjectError);
