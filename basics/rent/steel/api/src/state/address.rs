use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountType {
    AddressData = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AddressData {
    pub name_len: u32,
    pub name: [u8; 32],
    pub address_len: u32,
    pub address: [u8; 32],
}

account!(AccountType, AddressData);