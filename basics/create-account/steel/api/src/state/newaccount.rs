use steel::*;

use super::CreateAccountAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct NewAccount {
    pub userID: u8,
}

account!(CreateAccountAccount, NewAccount);
