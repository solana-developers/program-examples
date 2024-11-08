use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    AddressInfo = 0,
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
