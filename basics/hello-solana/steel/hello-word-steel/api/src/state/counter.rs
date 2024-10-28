use steel::*;

use super::MySteelProjectAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64 
}

account!(MySteelProjectAccount, Counter);
