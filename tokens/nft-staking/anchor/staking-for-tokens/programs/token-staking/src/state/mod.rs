use anchor_lang::prelude::*;

#[account]
pub struct StakingRules {
    pub authority: Pubkey,
    pub collection_address: Pubkey,
    pub reward_per_unix: f64,
    pub reward_mint: Pubkey,
    pub bump: u8,
}

impl StakingRules{
    pub fn space() -> usize {
        8 +     //  Discriminator
        32 +    //  Authority
        32 + 
        8 +
        32 +
        1
    }
}

#[account]
pub struct StakingAccount {
    pub owner: Pubkey,
    pub staking_rules: Pubkey,
    pub token_rewarded: f64,
    pub bump: u8
}

impl StakingAccount {
    pub fn space() -> usize {
        8 +
        32 +
        32 +
        8 +
        1
    }
}

#[account]
pub struct StakingInstance {
    pub staking_account: Pubkey,
    pub staking_rules: Pubkey,
    pub time: i64,    
    pub bump: u8
}

impl StakingInstance {
    pub fn space() -> usize {
        8 +
        32 +
        32 +
        8 +
        1
    }
}