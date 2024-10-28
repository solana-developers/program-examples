use crate::consts::*;
use steel::*;

/// Fetch PDA of the account.
pub fn new_account_pda() -> Option<(Pubkey, u8)> {
    Pubkey::try_find_program_address(&[CREATE_ACCOUNT], &crate::id())
}

/// This enum is used to get a discriminator
/// for the new account.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum CreateAccountDiscriminator {
    NewAccount = 0,
}

/// This empty struct represents the account type
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct NewAccount {
    pub user_id: u8,
}

account!(CreateAccountDiscriminator, NewAccount);
