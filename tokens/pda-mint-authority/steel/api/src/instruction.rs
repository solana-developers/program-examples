use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TokenInstruction {
    CreateToken = 0,
    MintToken = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateToken {
    pub token_name: [u8; 32],
    pub token_symbol: [u8; 10],
    pub token_uri: [u8; 64],
    pub bump: u8
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintToken {
    pub amount: u64,
}

instruction!(TokenInstruction, CreateToken);
instruction!(TokenInstruction, MintToken);