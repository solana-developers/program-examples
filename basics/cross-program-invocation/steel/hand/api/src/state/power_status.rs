use steel::*;

use super::HandAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct PowerStatus {
    pub is_on: u8,
}

account!(HandAccount, PowerStatus);
