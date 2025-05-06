use anchor_lang::prelude::*;

use solana_program::{*, program::{invoke, invoke_signed}};

use anchor_spl::{
    metadata::{MetadataAccount, MasterEditionAccount, Metadata, TokenRecordAccount},
    token::{Mint, TokenAccount, Token}, 
    associated_token::AssociatedToken,
};

use mpl_token_metadata::instruction::{
    builders::{Unlock, Revoke},
    UnlockArgs, InstructionBuilder, RevokeArgs,
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
        mint::authority = nft_master_edition,
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
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
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"token_record",
            signer_ata.key().as_ref(),
            ],
        seeds::program = token_metadata_program.key(),
        bump,
    )]
    pub nft_token_record: Account<'info, TokenRecordAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = nft_mint,
        associated_token::authority = signer,
    )]
    pub signer_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = sysvar::instructions::id())]
    /// CHECK: no need to check this
    pub sysvar_instructions: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(
        &mut self,
    ) -> Result<()> {

        // Unlock the NFT
        let bind = self.signer.key();
        let bind2 = self.nft_mint.key();

        let seeds = &[
            "instance".as_bytes(),
            bind.as_ref(),
            bind2.as_ref(),
            &[self.staking_instance.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let unlock_builder = Unlock {
            authority: self.staking_instance.key(),
            token_owner: Some(self.signer.key()),
            token: self.signer_ata.key(),
            mint: self.nft_mint.key(),
            metadata: self.nft_metadata.key(),
            edition: Some(self.nft_master_edition.key()),
            token_record: Some(self.nft_token_record.key()),
            payer: self.signer.key(),
            system_program: self.system_program.key(),
            sysvar_instructions: self.sysvar_instructions.key(),
            spl_token_program: Some(self.token_program.key()),
            authorization_rules_program: None,
            authorization_rules: None,
            args: UnlockArgs::V1 {
                authorization_data: None,
            },
        };

        let unlock_infos = vec![
            self.staking_instance.to_account_info(),
            self.signer.to_account_info(),
            self.signer_ata.to_account_info(),
            self.nft_mint.to_account_info(),
            self.nft_metadata.to_account_info(),
            self.nft_master_edition.to_account_info(),
            self.nft_token_record.to_account_info(),
            self.signer.to_account_info(),
            self.system_program.to_account_info(),
            self.sysvar_instructions.to_account_info(),
            self.token_program.to_account_info(),
        ];

        invoke_signed(&unlock_builder.instruction(), &unlock_infos, signer_seeds)?;

        // Revoke the delegation.
        let revoke_builder = Revoke {
            delegate_record: None,
            delegate: self.staking_instance.key(),
            metadata: self.nft_metadata.key(),
            master_edition: Some(self.nft_master_edition.key()),
            token_record: Some(self.nft_token_record.key()),
            mint: self.nft_mint.key(),
            token: Some(self.signer_ata.key()),
            authority: self.signer.key(),
            payer: self.signer.key(),
            system_program: self.system_program.key(),
            sysvar_instructions: self.sysvar_instructions.key(),
            spl_token_program: Some(self.token_program.key()),
            authorization_rules_program: None,
            authorization_rules: None,
            args: RevokeArgs::StakingV1
        };

        let revoke_infos = vec![
            self.staking_instance.to_account_info(),
            self.nft_metadata.to_account_info(),
            self.nft_master_edition.to_account_info(),
            self.nft_token_record.to_account_info(),
            self.nft_mint.to_account_info(),
            self.signer_ata.to_account_info(),
            self.signer.to_account_info(),
            self.system_program.to_account_info(),
            self.sysvar_instructions.to_account_info(),
            self.token_program.to_account_info(),
        ];

        invoke(&revoke_builder.instruction(), &revoke_infos)?;


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
