use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    TransferWithProgram = 1,
    TransferWithCPI = 2
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferWithProgram {
    pub amount: [u8; 8]
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferWithCPI {
    pub amount: [u8; 8]
}

instruction!(SteelInstruction, TransferWithProgram);
instruction!(SteelInstruction, TransferWithCPI);
