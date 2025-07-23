use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Pool {
    pub amm: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";
    pub const AUTHORITY_PREFIX: &'static str = "authority";
    pub const LIQUIDITY_PREFIX: &'static str = "liquidity";
    pub fn space() -> usize {
        // 32 bytes for amm (Pubkey)
        // 32 bytes for mint_a (Pubkey)
        // 32 bytes for mint_b (Pubkey)
        32 + 32 + 32
    }
}
