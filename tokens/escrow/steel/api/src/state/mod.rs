mod offer;

pub use offer::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    Offer = 0
}

/// Fetch PDA of the counter account.
pub fn offer_pda(maker: Pubkey, id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[OFFER, maker.as_ref(), &id.to_be_bytes()], &crate::id())
}
