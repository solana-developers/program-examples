use crate::consts::*;
use crate::utils::*;
use steel::*;

use super::ReallocAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AddressInfo {
    pub name: [u8; MAX_STR_LEN],
    pub house_number: u8,
    pub street: [u8; MAX_STR_LEN],
    pub city: [u8; MAX_STR_LEN],
}

impl AddressInfo {
    pub fn new(name: &str, house_number: u8, street: &str, city: &str) -> Self {
        AddressInfo {
            name: str_to_bytes(name),
            house_number,
            street: str_to_bytes(street),
            city: str_to_bytes(city),
        }
    }
}

account!(ReallocAccount, AddressInfo);
