use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TokenSwapInstruction {
    Initialize = 0,
    Add = 1
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Add {
    pub amount: [u8; 8]
}

instruction!(TokenSwapInstruction, Initialize);
instruction!(TokenSwapInstruction, Add);
