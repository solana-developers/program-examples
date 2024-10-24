use steel::*;

use super::SteelAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AccountToChange {}

account!(SteelAccount, AccountToChange);
