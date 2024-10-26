use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TokenSwapInstruction {
    CreateAmm = 0,
    CreatePool = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAmm {
    pub id: Pubkey,
    pub fee: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreatePool {}

instruction!(TokenSwapInstruction, CreateAmm);
instruction!(TokenSwapInstruction, CreatePool);
