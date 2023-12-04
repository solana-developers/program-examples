use anchor_lang::prelude::*;

mod context;
mod state;
mod errors;

use context::*;

declare_id!("GGWeSTDbz2WCERf59nTxwSRTFLMg7butvdk9tv4D53sN");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn create_staking_rule(ctx: Context<StakingRuleCreate>, decimals: u8, reward_per_unix: f64) -> Result<()> {
        ctx.accounts.create(decimals, reward_per_unix, &ctx.bumps)
    }

    pub fn create_staking_account(ctx: Context<StakingAccountCreate>) -> Result<()> {
        ctx.accounts.create(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }
}
