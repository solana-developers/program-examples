use steel::*;

use super::TransferSolAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64 
}

account!(TransferSolAccount, Counter);
