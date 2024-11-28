use steel::*;
use super::ReallocAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Message {
    pub message: [u8; 1024], // Max message size
    pub len: u32,            // Actual length of message
}

impl Message {
    pub fn required_space(message_len: usize) -> usize {
        std::mem::size_of::<Message>()
    }
}

account!(ReallocAccount, Message);