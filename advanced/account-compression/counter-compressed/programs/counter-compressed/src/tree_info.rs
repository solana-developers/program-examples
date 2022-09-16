use anchor_lang::prelude::*;

#[account]
pub struct TreeInfo {
    pub num_counters: u32,
}

pub const TREE_INFO_SIZE: usize = 8 + 4;
