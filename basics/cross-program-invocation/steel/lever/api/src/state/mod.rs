mod power_status;

pub use power_status::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum LeverAccount {
    PowerStatus = 0,
}

// /// Fetch PDA of the counter account.
// pub fn counter_pda() -> (Pubkey, u8) {
//     Pubkey::find_program_address(&[COUNTER], &crate::id())
// }
