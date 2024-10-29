use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum PdaRentPayerInstruction {
    InitializeRentVault = 0,
    CreateNewAccount = 1
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeRentVault {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DepositRent {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateNewAccount {}

instruction!(PdaRentPayerInstruction, InitializeRentVault);
instruction!(PdaRentPayerInstruction, CreateNewAccount);
