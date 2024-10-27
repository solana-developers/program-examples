// src/state.rs

use solana_program::pubkey::Pubkey;

pub struct TokenAccount {
    pub owner: Pubkey,
    pub balance: u64,
}
