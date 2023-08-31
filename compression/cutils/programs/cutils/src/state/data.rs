use crate::*;

pub const SEED_DATA: &[u8] = b"DATA";

#[account]
#[derive(Default, Debug)]
pub struct Data {
    /// The bump, used for PDA validation.
    pub bump: u8,
    pub tree: Pubkey,
    pub tree_nonce: u64,
}

impl Data {
    pub const LEN: usize = 8 + 1 + 32 + 8;

    pub fn new(bump: u8, tree: Pubkey, tree_nonce: u64) -> Self {
        Self {
            bump,
            tree,
            tree_nonce,
        }
    }
}
