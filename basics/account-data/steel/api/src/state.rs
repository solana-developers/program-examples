use steel::*;
use crate::consts::ADDRESS_INFO_SEED;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq)]
pub struct AddressInfoData {
    /// Name of the address owner (max 64 bytes)
    pub name: [u8; 64],
    /// House number as bytes
    pub house_number: [u8; 8],
    /// Street name (max 64 bytes)
    pub street: [u8; 64],
    /// City name (max 64 bytes)
    pub city: [u8; 64],
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

fn string_to_bytes(s: &str) -> [u8; 64] {
    let mut bytes = [0; 64];
    let s_bytes = s.as_bytes();
    let len = s_bytes.len().min(64);
    bytes[..len].copy_from_slice(&s_bytes[..len]);
    bytes
}

/// Account type discriminator
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AddressInfoAccount {
    AddressInfo = 0,
}

/// Account data structure
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AddressInfo {
    pub data: AddressInfoData,
}

// Link account discriminator with account data structure
account!(AddressInfoAccount, AddressInfo);

pub fn account_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ADDRESS_INFO_SEED], &crate::id())
}
