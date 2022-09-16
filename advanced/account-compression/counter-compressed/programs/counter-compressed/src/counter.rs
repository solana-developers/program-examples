use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

#[derive(Copy, Debug)]
#[account]
pub struct Counter {
    /// This identifies which tree it belongs to
    pub tree: Pubkey,
    /// This keeps track of which counter it is in the global state tree
    pub id: u32,
    /// Our normal count incrementer
    pub count: u64,
}

pub const COUNTER_PREFIX: &str = "counter";
pub const COUNTER_SIZE: usize = 8 + 32 + 4 + 8;

impl Counter {
    pub fn new(tree: &Pubkey, id: u32) -> Counter {
        Counter {
            tree: *tree,
            id,
            count: 0,
        }
    }
}

pub fn hash_counter(counter: Counter) -> Result<[u8; 32]> {
    Ok(keccak::hashv(&[&counter.try_to_vec()?]).to_bytes())
}
