use anchor_lang::prelude::*;

pub const MARKET_SEED: &[u8] = b"market";

// A Market is one trading pair (base/quote) with its own vaults and order book.
// The market PDA itself is the authority of the token vaults, so funds can only
// move out via program-signed CPIs (place/cancel/settle).
#[derive(InitSpace)]
#[account]
pub struct Market {
    pub authority: Pubkey,

    pub base_mint: Pubkey,

    pub quote_mint: Pubkey,

    pub base_vault: Pubkey,

    pub quote_vault: Pubkey,

    pub order_book: Pubkey,

    pub fee_basis_points: u16,

    pub tick_size: u64,

    pub min_order_size: u64,

    pub is_active: bool,

    pub bump: u8,
}
