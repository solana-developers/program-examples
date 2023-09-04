use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::AddressInfo;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct EnhancedAddressInfoExtender {
    pub state: String,
    pub zip: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct EnhancedAddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: u32,
}

impl EnhancedAddressInfo {
    pub fn from_address_info(address_info: AddressInfo, state: String, zip: u32) -> Self {
        EnhancedAddressInfo {
            name: address_info.name,
            house_number: address_info.house_number,
            street: address_info.street,
            city: address_info.city,
            state,
            zip,
        }
    }
}
