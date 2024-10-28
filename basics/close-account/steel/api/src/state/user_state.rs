use steel::*;

use super::{CloseAccountAccount, USER_SEED};

/// Fetch PDA of the counter account.
pub fn user_state_pda(user: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[USER_SEED, user.as_ref()], &crate::id())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct UserState {
    pub bump: u8,     // 1 byte
    pub user: Pubkey, // 32 bytes
    pub name: [u8; 64],
}

account!(CloseAccountAccount, UserState);
