use anchor_lang::prelude::*;

use crate::state::AddressInfo;


#[account]
pub struct EnhancedAddressInfoExtender {
    pub state: String,
    pub zip: u32,
}

#[account]
pub struct EnhancedAddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: u32,
}

impl EnhancedAddressInfo {

    pub const ACCOUNT_SPACE: usize = 8 + 40 + 1 + 40 + 40 + 40 + 4;

    pub fn from_address_info(
        address_info: AddressInfo,
        state: String,
        zip: u32,
    ) -> Self {

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