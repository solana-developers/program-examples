use quasar_lang::prelude::*;

/// Seed for the data account PDA.
pub const SEED_DATA: &[u8] = b"DATA";

/// Tracks the merkle tree and its nonce for minting.
#[account(discriminator = 1)]
pub struct Data {
    /// PDA bump seed.
    pub bump: u8,
    /// Padding for alignment.
    pub _padding: [u8; 7],
    /// The merkle tree address.
    pub tree: Address,
    /// Current nonce in the tree.
    pub tree_nonce: u64,
}
