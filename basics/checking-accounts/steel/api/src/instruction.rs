use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ValidationInstruction {
    CheckAccounts = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CheckAccountsArgs {}

instruction!(ValidationInstruction, CheckAccountsArgs);