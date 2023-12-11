use anchor_lang::prelude::*;

use anchor_spl::{
    metadata::{MetadataAccount, MasterEditionAccount, Metadata},
    token::Mint,
};

use crate::state::StakingRules;

#[derive(Accounts)]
pub struct StakingRuleCreate<'info> {
    #[account(
        init, 
        payer = initializer, 
        seeds = [b"rules", collection_mint.key().as_ref()], 
        bump,
        space = StakingRules::space()
    )]
    pub staking_rule: Account<'info, StakingRules>,

    #[account(
        mint::authority = collection_master_edition,
    )]
    pub collection_mint: Account<'info, Mint>,
    #[account(
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        seeds::program = token_metadata_program.key(),
        bump,
    )]
    pub collection_metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition",
            ],
        seeds::program = token_metadata_program.key(),
        bump,
    )]
    pub collection_master_edition: Account<'info, MasterEditionAccount>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakingRuleCreate<'info> {
    pub fn create(
        &mut self,
        reward_per_unix: f64,
        bumps: &StakingRuleCreateBumps
    ) -> Result<()> {

        self.staking_rule.authority = self.initializer.key();
        self.staking_rule.collection_address = self.collection_mint.key();
        self.staking_rule.reward_per_unix = reward_per_unix;
        self.staking_rule.bump = bumps.staking_rule;

        Ok(())
    }
}