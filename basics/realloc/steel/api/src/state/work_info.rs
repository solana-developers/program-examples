use crate::consts::*;
use crate::utils::*;
use steel::*;

use super::ReallocAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct WorkInfo {
    pub name: [u8; MAX_STR_LEN],
    pub position: [u8; MAX_STR_LEN],
    pub company: [u8; MAX_STR_LEN],
    pub years_employed: u8,
}

impl WorkInfo {
    pub fn new(name: &str, position: &str, company: &str, years_employed: u8) -> Self {
        WorkInfo {
            name: str_to_bytes(name),
            position: str_to_bytes(position),
            company: str_to_bytes(company),
            years_employed,
        }
    }
}

account!(ReallocAccount, WorkInfo);
