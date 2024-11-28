use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TransferSolInstruction {
    WithCpi = 0,
    WithProgram = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WithCpi {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WithProgram {
    pub amount: [u8; 8],
}

instruction!(TransferSolInstruction, WithCpi);
instruction!(TransferSolInstruction, WithProgram);
