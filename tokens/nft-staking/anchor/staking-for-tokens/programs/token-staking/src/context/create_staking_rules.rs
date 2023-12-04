use anchor_lang::prelude::*;

use anchor_spl::{
    metadata::{MetadataAccount, MasterEditionAccount, Metadata},
    token::{Mint, Token},
};

use crate::{state::StakingRules, errors::StakingErrors};

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct StakingRuleCreate<'info> {
    #[account(
        init, 
        payer = initializer, 
        seeds = [b"rules", collection_mint.key().as_ref()], 
        bump,
        space = StakingRules::space()
    )]
    pub staking_rules: Account<'info, StakingRules>,

     // Create new mint account
     #[account(
        init_if_needed,
        payer = initializer,
        mint::decimals = decimals,
        mint::authority = staking_rules.key(),
    )]
    pub reward_mint: Account<'info, Mint>,

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
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakingRuleCreate<'info> {
    pub fn create(
        &mut self,
        decimals: u8,
        reward_per_unix: f64,
        bumps: &StakingRuleCreateBumps
    ) -> Result<()> {
        
    // Check if the decimal places of reward_per_unix exceed the decimals of the mint
    let num_str = reward_per_unix.to_string();
    let decimal_places = if let Some(dot_pos) = num_str.find('.') {
        let decimals_str = &num_str[dot_pos + 1..];
        decimals_str.trim_end_matches('0').len()
    } else {
        0
    };

    require!(decimal_places as u8 <= decimals, StakingErrors::InvalidDecimals);
    
    // Initialize the staking rules
    self.staking_rules.authority = self.initializer.key();
    self.staking_rules.collection_address = self.collection_mint.key();
    self.staking_rules.reward_per_unix = reward_per_unix;
    self.staking_rules.reward_mint = self.reward_mint.key();
    self.staking_rules.bump = bumps.staking_rules;

    Ok(())
     
    }
}