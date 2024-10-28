use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ReallocInstruction {
    Initialize = 0,
    Update = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {
    pub message: [u8; 1024],
    pub len: [u8; 4],  // Store as bytes like in escrow
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Update {
    pub message: [u8; 1024],
    pub len: [u8; 4],
}

instruction!(ReallocInstruction, Initialize);
instruction!(ReallocInstruction, Update);