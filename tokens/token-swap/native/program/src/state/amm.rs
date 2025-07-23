use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Amm {
    pub admin: Pubkey,
    pub fee: u16,
}

impl Amm {
    pub const SEED_PREFIX: &'static str = "amm";
    pub fn space() -> usize {
        // 32 bytes for admin (Pubkey)
        // 2 bytes for fee (u16)
        32 + 2
    }
}
