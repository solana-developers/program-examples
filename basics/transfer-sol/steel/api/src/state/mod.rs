mod counter;

pub use counter::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    Counter = 0
}

/// Fetch PDA of the counter account.
pub fn counter_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER], &crate::id())            
}
