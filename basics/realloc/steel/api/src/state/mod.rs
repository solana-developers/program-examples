mod address_info;
mod enchanced_address_info;
mod work_info;

pub use address_info::*;
pub use enchanced_address_info::*;
pub use work_info::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum ReallocAccount {
    AddressInfo = 0,
    EnhancedAddressInfo = 1,
    EnhancedAddressInfoExtender = 2,
    WorkInfo = 3,
}

/// Fetch PDA of the counter account.
pub fn counter_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER], &crate::id())
}
