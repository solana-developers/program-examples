use std::str;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    Create = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Create {
    pub token_name: [u8; 32],
    pub token_symbol: [u8; 8],
    pub token_uri: [u8; 64],
    pub decimals: u8,
}

instruction!(SteelInstruction, Create);
