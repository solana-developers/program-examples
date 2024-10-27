use steel::*;
use super::RentAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Address {
    pub name: [u8; 32],
    pub address: [u8; 64],
}

account!(RentAccount, Address);