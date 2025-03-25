use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TokenInstruction {
    CreateToken = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateToken {
    pub name: [u8; 32],
    pub symbol: [u8; 8],
    pub uri: [u8; 128],
    pub decimals: u8,
}

instruction!(TokenInstruction, CreateToken);
