use anchor_lang::prelude::*;

use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program::{
    Transfer,
    transfer, 
};
use anchor_spl::token::spl_token::instruction::transfer as spl_transfer;
use anchor_spl::associated_token::{
    get_associated_token_address, 
    AssociatedToken
};
use anchor_spl::metadata::{
    MasterEditionAccount, 
    Metadata, 
    MetadataAccount
};
use anchor_spl::token::{
    burn, 
    Burn, 
    Mint, 
    Token, TokenAccount
};
use mpl_bubblegum::instructions::MintToCollectionV1CpiBuilder;
use mpl_bubblegum::types::{
    Collection, 
    MetadataArgs, 
    TokenProgramVersion, 
    TokenStandard
};
use mpl_bubblegum::ID as BUBBLEGUM_ID;
use spl_account_compression::ID as SPL_ACCOUNT_COMPRESSION_ID;
use spl_noop::ID as SPL_NOOP_ID;

use crate::state::TreeStatus;
use crate::{
    state::Config, 
    CustomError
};

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub authority: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub allow_mint: Option<Account<'info, Mint>>,
    #[account(mut)]
    pub allow_mint_ata: Option<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"collection", config.key().as_ref()],
        bump,
    )]
    pub collection: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            collection.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub collection_metadata: Account<'info, MetadataAccount>,
    #[account(
        mut,
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            collection.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub collection_edition: Account<'info, MasterEditionAccount>,
    /// CHECK: Tree Config account that will be checked by the Bubblegum Program
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    /// CHECK: Merkle Tree account that will be checked by the Bubblegum Program
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: SPL NOOP Program checked by the corresponding address
    #[account(address = SPL_NOOP_ID)]
    pub log_wrapper: UncheckedAccount<'info>,
    /// CHECK: Bubblegum Program checked by the corresponding address
    #[account(address = BUBBLEGUM_ID)]
    pub bubblegum_program: UncheckedAccount<'info>,
    /// CHECK: SPL Account Compression Program checked by the corresponding address
    #[account(address = SPL_ACCOUNT_COMPRESSION_ID)]
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> MintNFT<'info> {
    pub fn mint_cnft(&mut self, name: String, symbol: String, uri: String, pay_sol: bool, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {

        // Check if the Candy Machine is active
        require!(self.config.status != TreeStatus::Inactive, CustomError::CandyMachineInactive);

        // Check if the Candy Machine is private
        if self.config.status == TreeStatus::Private {
            // Check if there is an Allow Mint account and Allow Mint ATA account
            if let (Some(allow_mint), Some(allow_mint_ata)) = (&self.allow_mint, &self.allow_mint_ata) {

                // Check if the Allow Mint account is the same as the one in the config
                require!(allow_mint.key() == self.config.allow_mint.unwrap(), CustomError::InvalidAllowMint);

                // Check if the Allow Mint ATA account belongs to the user
                let ata_address = get_associated_token_address(&self.user.key(), &allow_mint.key());
                require!(ata_address == self.allow_mint_ata.as_ref().unwrap().key(), CustomError::InvalidAllowMintATA);

                // Burn the Allow Mint token
                let cpi_program = self.token_program.to_account_info();
                let cpi_accounts = Burn {
                    mint: allow_mint.to_account_info(),
                    from: allow_mint_ata.to_account_info(),
                    authority: self.user.to_account_info(),
                };
                let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
                burn(cpi_context, 1 * 10_u32.pow(allow_mint.decimals as u32) as u64)?;
            }
            else {
                // If the Candy Machine is private and there is no Allow Mint account, check if the user is in the Allow List
                self.config.allow_list.iter().find(|x| x.user == self.user.key()).ok_or(CustomError::UserNotAllowed)?;

                // Check if the user has already claimed
                let user_struct = self.config.allow_list.iter_mut().find(|x| x.user == self.user.key()).unwrap();      
                if user_struct.amount == 0 {
                    return Err(CustomError::AlreadyClaimed.into());
                }
                // Decrease the allowed amount of the user
                user_struct.amount -= 1;    
            }
        }

        // Create signer seeds for the CPI calls
        let seeds = &[
            &b"config"[..], 
            &self.authority.key.as_ref(),
            &[self.config.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // CPI call to the Bubblegum Program to mint the cNFT
        MintToCollectionV1CpiBuilder::new(&self.bubblegum_program.to_account_info())
            .tree_config(&self.tree_config.to_account_info())
            .leaf_owner(&self.user.to_account_info())
            .leaf_delegate(&self.user)
            .merkle_tree(&self.merkle_tree.to_account_info())
            .payer(&self.user.to_account_info())
            .tree_creator_or_delegate(&self.config.to_account_info())
            .collection_authority(&self.config.to_account_info())
            .collection_authority_record_pda(None)
            .collection_mint(&self.collection.to_account_info())
            .collection_metadata(&self.collection_metadata.to_account_info())
            .collection_edition(&self.collection_edition.to_account_info())
            .bubblegum_signer(&self.config.to_account_info())
            .log_wrapper(&self.log_wrapper.to_account_info())
            .compression_program(&self.compression_program.to_account_info())
            .token_metadata_program(&self.metadata_program.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .metadata(
                MetadataArgs {
                    name,
                    symbol,
                    uri,
                    creators: vec![],
                    seller_fee_basis_points: 0,
                    primary_sale_happened: false,
                    is_mutable: false,
                    edition_nonce: Some(0),
                    uses: None,
                    collection: Some(Collection {
                        verified: true,
                        key: self.collection.key(),
                    }),
                    token_program_version: TokenProgramVersion::Original,
                    token_standard: Some(TokenStandard::NonFungible),
                }
            )
        .invoke_signed(signer_seeds)?;

        // Check if the user wants to pay in SOL or SPL and if there is a price in SOL or SPL. Return an error if the settings are invalid
        match pay_sol {
            // If the user wants to pay in SOL, check if there is a price in SOL. 
            // If there is, transfer the SOL to the authority, otherwise check if there is a price in SPL and return an error if there is
            true => match self.config.price_sol.is_some() {
                true => self.transfer_sol()?,
                false => require!(self.config.price_spl.is_none(), CustomError::InvalidSPLSettings),
            },
            // If the user wants to pay in SPL, check if there is a price in SPL and an SPL address. 
            // If there is, transfer the SPL to the authority, otherwise check if there is a price in SOL and return an error if there is
            false => match self.config.price_spl.is_some() && self.config.spl_address.is_some() {
                true => self.transfer_spl(remaining_accounts)?,
                false => require!(self.config.price_sol.is_none(), CustomError::InvalidSPLSettings),
            },
        }

        // Increase the current supply
        self.config.current_supply += 1;

        // If the total supply is equal to the current supply, close the account
        if self.config.current_supply >= self.config.total_supply {
            self.close_account()?;
        }

        Ok(())
    }

    pub fn transfer_sol(&mut self) -> Result<()> {
        // Transfer the SOL to the authority
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.authority.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_context, self.config.price_sol.unwrap())
    }

    pub fn transfer_spl(&mut self, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {
        // Check if there are 2 remaining accounts
        if remaining_accounts.len() != 2 {
            return Err(CustomError::InvalidRemainingAccounts.into());
        }

        // Get the expected ATA accounts
        let expected_from_ata = get_associated_token_address(&self.user.key, &self.config.spl_address.as_ref().unwrap());
        let expected_to_ata = get_associated_token_address(&self.authority.key, &self.config.spl_address.as_ref().unwrap());

        // Check if the first remaining accounts are is the expected source ATA
        require_keys_eq!(remaining_accounts[0].key(), expected_from_ata, CustomError::InvalidSourceRemainingAccount);

        // Check if the second remaining account is the expected destination ATA
        require_keys_eq!(remaining_accounts[1].key(), expected_to_ata, CustomError::InvalidDestinationRemainingAccount);


        // Create the transfer instruction
        let transfer_tokens_instruction = spl_transfer(
            &self.token_program.key,
            &remaining_accounts[0].key(),
            &remaining_accounts[1].key(),
            &self.user.key(),
            &[&self.user.key()],
            self.config.price_spl.unwrap(),
        )?;
        
        // Collect the required accounts for the transfer
        let required_accounts_for_transfer = [
            remaining_accounts[0].to_account_info().clone(),
            remaining_accounts[1].to_account_info().clone(),
            self.user.to_account_info().clone(),
        ];
        
        // Invoke the transfer instruction
        invoke(
            &transfer_tokens_instruction,
            &required_accounts_for_transfer,
        )?;

        Ok(())
    }

    pub fn close_account(&mut self) -> Result<()> {
        // Close the config account and transfer the rent lamports to the authority
        **self.authority.lamports.borrow_mut() = self.authority.lamports().checked_add(self.config.to_account_info().lamports()).unwrap();
        **self.config.to_account_info().lamports.borrow_mut() = 0;
        
        Ok(())
    }
}