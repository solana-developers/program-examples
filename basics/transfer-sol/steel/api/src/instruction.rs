use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TransferInstruction {
    TransferSolWithCpi = 0,
    TransferSolWithProgram = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferSolWithCpi {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferSolWithProgram {
    pub amount: u64,
}

instruction!(TransferInstruction, TransferSolWithCpi);
instruction!(TransferInstruction, TransferSolWithProgram);