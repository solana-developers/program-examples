use steel::*;


#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ReallocInstruction {
    Initialize = 0,
    Update = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MessageArgs {
    pub message_len: u32,
    pub message: [u8; 1024],
}

instruction!(ReallocInstruction, MessageArgs);