use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TransferInstruction {
    TransferSolWithCpi = 0,
    TransferSolWithProgram = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferArgs {
    pub amount: u64,
}

instruction!(TransferInstruction, TransferArgs);