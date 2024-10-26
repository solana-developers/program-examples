use steel::*;

use crate::consts::{AUTHORITY_SEED, LIQUIDITY_SEED};

use super::TokenSwapAccount;

/// Fetch PDA of the pool account.
pub fn pool_pda(amm: Pubkey, mint_a: Pubkey, mint_b: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[amm.as_ref(), mint_a.as_ref(), mint_b.as_ref()],
        &crate::id(),
    )
}

pub fn pool_authority_pda(amm: Pubkey, mint_a: Pubkey, mint_b: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            amm.as_ref(),
            mint_a.as_ref(),
            mint_b.as_ref(),
            AUTHORITY_SEED,
        ],
        &crate::id(),
    )
}

pub fn mint_liquidity_pda(amm: Pubkey, mint_a: Pubkey, mint_b: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            amm.as_ref(),
            mint_a.as_ref(),
            mint_b.as_ref(),
            LIQUIDITY_SEED,
        ],
        &crate::id(),
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Pool {
    /// Primary key of the AMM
    pub amm: Pubkey,

    /// Mint of token A
    pub mint_a: Pubkey,

    /// Mint of token B
    pub mint_b: Pubkey,
}

account!(TokenSwapAccount, Pool);
