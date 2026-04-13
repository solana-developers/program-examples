use anchor_lang::prelude::*;

use crate::{ABWallet, Config};

#[derive(Accounts)]
pub struct RemoveWallet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        mut,
        close = authority,
    )]
    pub ab_wallet: Account<'info, ABWallet>,

    pub system_program: Program<'info, System>,
}

impl RemoveWallet<'_> {
    pub fn remove_wallet(&mut self) -> Result<()> {
        Ok(())
    }
}
