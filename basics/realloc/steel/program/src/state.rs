use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    AddressInfo,
    ExtendedAddressInfo,
}

account!(SteelAccount, AddressInfo);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AddressInfo {
    pub name: [u8; 48],
    pub house_number: u8,
    pub street: [u8; 48],
    pub city: [u8; 48],
}

account!(SteelAccount, ExtendedAddressInfo);
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ExtendedAddressInfo {
    pub name: [u8; 48],
    pub house_number: u8,
    pub street: [u8; 48],
    pub city: [u8; 48],
    pub state: [u8; 48],
    pub zip: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EnhancedAddressInfoExtender {
    pub state: [u8; 48],
    pub zip: u32,
}
