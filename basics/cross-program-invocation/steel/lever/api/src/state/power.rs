use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountType {
    PowerStatus = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct PowerStatus {
    pub is_on: u8, // Using u8 instead of bool for Pod compatibility
}

account!(AccountType, PowerStatus);
