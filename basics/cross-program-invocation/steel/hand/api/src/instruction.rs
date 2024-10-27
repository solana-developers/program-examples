use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum HandInstruction {
    Initialize = 0,
    SetPowerStatus = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetPowerStatus {
    pub name: [u8; 32],
}

instruction!(HandInstruction, Initialize);
instruction!(HandInstruction, SetPowerStatus);
