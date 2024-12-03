mod offer;

use crate::consts::*;
pub use offer::*;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAccount {
    Offer = 0,
}

/// Fetch PDA of the offer account.
pub fn offer_pda(maker: Pubkey, id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[OFFER, maker.as_ref(), id.to_le_bytes().as_ref()],
        &crate::id(),
    )
}
