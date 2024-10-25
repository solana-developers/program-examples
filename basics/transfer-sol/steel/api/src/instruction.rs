use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TransferSolInstruction {
    TransferSolWithCpi = 0,
    TransferSolWithProgram = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferSolWithCpi {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferSolWithProgram {
    pub amount: [u8; 8],
}

instruction!(TransferSolInstruction, TransferSolWithCpi);
instruction!(TransferSolInstruction, TransferSolWithProgram);
