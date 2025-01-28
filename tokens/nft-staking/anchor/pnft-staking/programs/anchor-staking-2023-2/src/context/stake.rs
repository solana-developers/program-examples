use anchor_lang::prelude::*;

use solana_program::{*, program::{invoke, invoke_signed}};

use anchor_spl::{
    metadata::{MetadataAccount, MasterEditionAccount, Metadata, TokenRecordAccount},
    token::{Mint, TokenAccount, Token}, 
    associated_token::AssociatedToken,
};

use mpl_token_metadata::instruction::{
    builders::{Delegate, Lock},
    DelegateArgs, InstructionBuilder, LockArgs,
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

impl<'info> Stake<'info> {
    pub fn stake(
        &mut self,
        bumps: &StakeBumps
    ) -> Result<()> {

        // Delegate the NFT
        let delegate_builder = Delegate {
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
            args: DelegateArgs::StakingV1 {
                amount: 1,
                authorization_data: None,
            },
        };

        let delegate_infos = vec![
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

        invoke(&delegate_builder.instruction(), &delegate_infos)?;

        let bind = self.signer.key();
        let bind2 = self.nft_mint.key();

        // Freeze the NFT.
        let seeds = &[
            "instance".as_bytes(),
            bind.as_ref(),
            bind2.as_ref(),
            &[bumps.staking_instance]
        ];
        let signer_seeds = &[&seeds[..]];
        

        let lock_builder = Lock {
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
            args: LockArgs::V1 {
                authorization_data: None,
            },
        };
 
        let lock_infos = vec![
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
 
        invoke_signed(&lock_builder.instruction(), &lock_infos, signer_seeds)?;

        // Populate the Staking Instance
        self.staking_instance.staking_account = self.staking_account.key();
        self.staking_instance.staking_rules = self.staking_rules.key();
        self.staking_instance.time = Clock::get().unwrap().unix_timestamp;
        self.staking_instance.bump = bumps.staking_instance;
        
        Ok(())
    }
}