use anchor_lang::prelude::*;

use anchor_spl::{
    metadata::{MetadataAccount, MasterEditionAccount, Metadata, ThawDelegatedAccount, thaw_delegated_account},
    token::{Mint, TokenAccount, Token, Revoke, revoke, MintTo, mint_to}, 
    associated_token::AssociatedToken,
};

use crate::{state::{StakingRules, StakingAccount, StakingInstance}, errors::StakingErrors};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        mut, 
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
        close = signer,
        seeds = [b"instance", signer.key().as_ref(), nft_mint.key().as_ref()], 
        bump = staking_instance.bump,
        constraint = staking_instance.staking_account == staking_account.key() @StakingErrors::InvalidStakingAccount,
        constraint = staking_instance.staking_rules == staking_rules.key() @StakingErrors::InvalidStakingRules,
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    #[account(
        mut,
        constraint = staking_rules.reward_mint == reward_mint.key() @StakingErrors::InvalidRewardMint,
    )]
    pub reward_mint: Account<'info, Mint>,

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
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = nft_mint,
        associated_token::authority = signer,
    )]
    pub nft_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = reward_mint,
        associated_token::authority = signer,
    )]
    pub token_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(
        &mut self,
    ) -> Result<()> {

        // Unlock the NFT
        let seeds = &[
            "rules".as_bytes(),
            self.staking_rules.collection_address.as_ref(),
            &[self.staking_rules.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = ThawDelegatedAccount {
            metadata: self.nft_metadata.to_account_info(),
            delegate: self.staking_rules.to_account_info(),
            token_account: self.nft_ata.to_account_info(),
            edition: self.nft_master_edition.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        thaw_delegated_account(cpi_context)?;

        // Revoke the delegation.
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Revoke {
            source: self.nft_ata.to_account_info(),
            authority:self.signer.to_account_info(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);  

        revoke(cpi_context)?;

        let time_now = Clock::get().unwrap().unix_timestamp;  
        let time_passed: i64 = time_now.checked_sub(self.staking_instance.time).unwrap();
        let rewards: f64 = (time_passed as f64) * self.staking_rules.reward_per_unix;
        let scaled_rewards = rewards * 10f64.powi(self.reward_mint.decimals as i32);

        // Update the Staking Instance
        self.staking_instance.time = time_now;

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.token_ata.to_account_info(),
            authority: self.staking_rules.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_context,  scaled_rewards as u64)?;

        // Update the Staking Account
        self.staking_account.token_rewarded += rewards;  

        Ok(())
    }
}
