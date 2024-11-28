use steel::*;
mod message;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum ReallocAccount {
    Message = 0,
}

pub use message::*;