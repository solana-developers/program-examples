use crate::consts::*;
use crate::state::AddressInfo;
use crate::utils::*;
use steel::*;

use super::ReallocAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EnhancedAddressInfoExtender {
    pub state: [u8; MAX_STR_LEN],
    pub zip: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EnhancedAddressInfo {
    pub zip: u32,
    pub house_number: u8,

    pub name: [u8; MAX_STR_LEN],
    pub street: [u8; MAX_STR_LEN],
    pub city: [u8; MAX_STR_LEN],
    pub state: [u8; MAX_STR_LEN],

    pub _padding: [u8; 3],
}

impl EnhancedAddressInfo {
    pub fn from_address_info(address_info: AddressInfo, state: &str, zip: u32) -> Self {
        EnhancedAddressInfo {
            name: address_info.name,
            house_number: address_info.house_number,
            street: address_info.street,
            city: address_info.city,
            state: str_to_bytes(state),
            zip,
            _padding: [0u8; 3],
        }
    }
}

account!(ReallocAccount, EnhancedAddressInfo);
