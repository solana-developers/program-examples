use steel::*;

use super::CheckingAccountsAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64 
}

account!(CheckingAccountsAccount, Counter);
