use crate::consts::*;
use steel::*;

/// Fetch PDA of the counter account.
pub fn new_account_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CREATE_ACCOUNT], &crate::id())
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum CreateAccountDiscriminator {
    NewAccount = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct NewAccount {
    pub user_id: u8,
}

account!(CreateAccountDiscriminator, NewAccount);
