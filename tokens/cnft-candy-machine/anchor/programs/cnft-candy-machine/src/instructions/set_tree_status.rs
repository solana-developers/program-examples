use anchor_lang::prelude::*;

use crate::state::{
    Config, 
    TreeStatus
};

#[derive(Accounts)]
pub struct SetTreeStatus<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
}

impl<'info> SetTreeStatus<'info> {
    pub fn set_tree_status(&mut self, status: TreeStatus) -> Result<()> {
        // Set the tree status
        self.config.status = status;
        Ok(())
    }
}