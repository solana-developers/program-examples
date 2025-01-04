use steel::*;

use crate::consts::COUNTER_SEED;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum CounterAccount {
    Counter = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64,
}

account!(CounterAccount, Counter);

/// Fetch PDA of the counter account.
pub fn counter_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER_SEED], &crate::id())
}
