mod power;

pub use power::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum LeverAccount {
    Counter = 0
}

/// Fetch PDA of the counter account.
pub fn counter_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER], &crate::id())            
}
