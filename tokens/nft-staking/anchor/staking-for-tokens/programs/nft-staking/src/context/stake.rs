use anchor_lang::prelude::*;

use anchor_spl::{
    metadata::{MetadataAccount, MasterEditionAccount, Metadata, FreezeDelegatedAccount, freeze_delegated_account},
    token::{Mint, TokenAccount, Token, Approve, approve}, 
    associated_token::AssociatedToken,
};

use crate::{state::{StakingRules, StakingAccount, StakingInstance}, errors::StakingErrors};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut, 
        seeds = [b"rules", staking_rules.collection_address.as_ref()], 
        bump = staking_rules.bump,
    )]
    pub staking_rules: Account<'info, StakingRules>,

    #[account(
        seeds = [b"account", staking_rules.key().as_ref(), signer.key().as_ref()], 
        bump = staking_account.bump,
        constraint = staking_account.staking_rules == staking_rules.key() @StakingErrors::InvalidStakingRules,
        constraint = staking_account.owner == signer.key() @StakingErrors::InvalidOwner,
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(
        init,
        payer = signer,
        seeds = [b"instance", signer.key().as_ref(), nft_mint.key().as_ref()], 
        bump,
        space = StakingInstance::space()
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
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = nft_mint,
        associated_token::authority = signer,
    )]
    pub nft_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(
        &mut self,
        bumps: &StakeBumps
    ) -> Result<()> {

        // Delegate the NFT
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            to: self.nft_ata.to_account_info(),
            delegate: self.staking_rules.to_account_info(),
            authority:self.signer.to_account_info(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);  

        approve(cpi_context, 1)?;

        // Freeze the NFT.
        let seeds = &[
            "rules".as_bytes(),
            self.staking_rules.collection_address.as_ref(),
            &[self.staking_rules.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = FreezeDelegatedAccount {
            metadata: self.nft_metadata.to_account_info(),
            delegate: self.staking_rules.to_account_info(),
            token_account: self.nft_ata.to_account_info(),
            edition: self.nft_master_edition.to_account_info(), // is it the master edition?
            mint: self.nft_mint.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        freeze_delegated_account(cpi_context)?;

        // Populate the Staking Instance
        self.staking_instance.staking_account = self.staking_account.key();
        self.staking_instance.staking_rules = self.staking_rules.key();
        self.staking_instance.time = Clock::get().unwrap().unix_timestamp;
        self.staking_instance.bump = bumps.staking_instance;
        
        Ok(())
    }
}