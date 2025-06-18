use steel::*;

use super::SteelAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64,
}

account!(SteelAccount, Counter);
