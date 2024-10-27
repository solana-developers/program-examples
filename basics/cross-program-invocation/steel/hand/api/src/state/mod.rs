mod power_status;

pub use power_status::*;

use steel::*;

// use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum HandAccount {
    PowerStatus = 0,
}

// Fetch PDA of the counter account.
// pub fn counter_pda() -> (Pubkey, u8) {
//     Pubkey::find_program_address(&[COUNTER], &crate::id())
// }
