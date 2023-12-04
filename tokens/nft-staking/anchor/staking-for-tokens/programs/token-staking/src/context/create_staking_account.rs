use anchor_lang::prelude::*;

use crate::state::{StakingRules, StakingAccount};

#[derive(Accounts)]
pub struct StakingAccountCreate<'info> {
    #[account(
        seeds = [b"rules", staking_rules.collection_address.as_ref()], 
        bump = staking_rules.bump,
    )]
    pub staking_rules: Account<'info, StakingRules>,

    #[account(
        init, 
        payer = signer, 
        seeds = [b"account", staking_rules.key().as_ref(), signer.key().as_ref()], 
        bump,
        space = StakingAccount::space()
    )]
    pub staking_account: Account<'info, StakingAccount>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakingAccountCreate<'info> {
    pub fn create(
        &mut self,
        bumps: &StakingAccountCreateBumps
    ) -> Result<()> {

        self.staking_account.owner = self.signer.key();
        self.staking_account.staking_rules = self.staking_rules.key();
        self.staking_account.token_rewarded = 0.0;
        self.staking_account.bump = bumps.staking_account;

        Ok(())
    }
}