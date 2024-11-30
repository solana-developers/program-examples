use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum AccountDataInstruction {
    Initialize = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {
    pub name: [u8; 64],
    pub house_number: u8,
    pub street: [u8; 64],
    pub city: [u8; 64],
}

instruction!(AccountDataInstruction, Initialize);
