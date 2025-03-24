mod token;

pub use token::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum TokenAccount {
    Token = 0
}

/// Fetch PDA of the token metadata account.
pub fn metadata_pda(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[METADATA, mpl_token_metadata::ID.as_ref(), mint.as_ref()],  &mpl_token_metadata::ID)            
}
