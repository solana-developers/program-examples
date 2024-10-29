use steel::*;

use super::TokenSwapAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64 
}

account!(TokenSwapAccount, Counter);
