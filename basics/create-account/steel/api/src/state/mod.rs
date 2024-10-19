mod newaccount;

pub use newaccount::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum CreateAccountAccount {
    NewAccount = 0
}

/// Fetch PDA of the new account.
pub fn new_account_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[NEWACCOUNT], &crate::id())            
}
