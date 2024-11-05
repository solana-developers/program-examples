use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    Init = 0,
    Create = 1,
    Mint = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Init {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Create {
    pub token_name: [u8; 32],
    pub token_symbol: [u8; 8],
    pub token_uri: [u8; 64],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Mint {
    pub amount: [u8; 8],
}

instruction!(SteelInstruction, Init);
instruction!(SteelInstruction, Create);
instruction!(SteelInstruction, Mint);
