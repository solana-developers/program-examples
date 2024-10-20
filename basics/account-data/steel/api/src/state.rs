use steel::*;

use crate::consts::ADDRESS_INFO_SEED;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq)]
pub struct AddressInfoData {
    pub name: [u8; 64],
    pub house_number: [u8; 8],
    pub street: [u8; 64],
    pub city: [u8; 64],
}

fn string_to_bytes(s: &str) -> [u8; 64] {
    let mut bytes = [0; 64];
    let s_bytes = s.as_bytes();
    let len = s_bytes.len().min(64);
    bytes[..len].copy_from_slice(&s_bytes[..len]);
    bytes
}

impl AddressInfoData {
    pub fn new(name: String, house_number: u64, street: String, city: String) -> Self {
        Self {
            name: string_to_bytes(&name),
            house_number: house_number.to_le_bytes(),
            street: string_to_bytes(&street),
            city: string_to_bytes(&city),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AddressInfoAccount {
    AddressInfo = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AddressInfo {
    pub data: AddressInfoData,
}

account!(AddressInfoAccount, AddressInfo);

/// Fetch PDA of the address info account.
pub fn account_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ADDRESS_INFO_SEED], &crate::id())
}
