mod account_to_change;

pub use account_to_change::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    AccountToChange = 0,
}

/// Fetch PDA of the account_to_change account.
pub fn account_to_change_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ACCOUNT_TO_CHANGE], &crate::id())
}
