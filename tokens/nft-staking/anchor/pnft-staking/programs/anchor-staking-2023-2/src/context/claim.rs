use anchor_lang::prelude::*;

use anchor_spl::{token::Mint, metadata::{MetadataAccount, MasterEditionAccount, Metadata}};

use crate::{state::{StakingRules, StakingAccount, StakingInstance}, errors::StakingErrors};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(
        seeds = [b"rules", staking_rules.collection_address.as_ref()], 
        bump = staking_rules.bump,
    )]
    pub staking_rules: Account<'info, StakingRules>,

    #[account(
        mut,
        seeds = [b"account", staking_rules.key().as_ref(), signer.key().as_ref()], 
        bump = staking_account.bump,
        constraint = staking_account.staking_rules == staking_rules.key() @StakingErrors::InvalidStakingRules,
        constraint = staking_account.owner == signer.key() @StakingErrors::InvalidOwner,
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(
        mut,
        seeds = [b"instance", signer.key().as_ref(), nft_mint.key().as_ref()], 
        bump = staking_instance.bump,
        constraint = staking_instance.staking_account == staking_account.key() @StakingErrors::InvalidStakingAccount,
        constraint = staking_instance.staking_rules == staking_rules.key() @StakingErrors::InvalidStakingRules,
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    #[account(
        mint::authority = nft_master_edition,
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program = token_metadata_program.key(),
        bump,
        constraint = nft_metadata.collection.is_some(),
        constraint = nft_metadata.collection.as_ref().unwrap().verified,
        constraint = nft_metadata.collection.as_ref().unwrap().key == staking_rules.collection_address @StakingErrors::InvalidCollection,
    )]
    pub nft_metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition",
            ],
        seeds::program = token_metadata_program.key(),
        bump,
    )]
    pub nft_master_edition: Account<'info, MasterEditionAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(
        &mut self,
    ) -> Result<()> {

        let time_now = Clock::get().unwrap().unix_timestamp;  
        let time_passed: i64 = time_now.checked_sub(self.staking_instance.time).unwrap();

        let rewards: f64 = (time_passed as f64) * self.staking_rules.reward_per_unix;

        // Update the Staking Instance
        self.staking_instance.time = time_now;

        // Update the Staking Account
        self.staking_account.point_rewarded = self.staking_account.point_rewarded.checked_add(rewards.floor() as u64).unwrap(); 


        Ok(())
    }
}