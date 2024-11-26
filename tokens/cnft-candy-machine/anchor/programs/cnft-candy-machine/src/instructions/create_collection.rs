use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::{
        mpl_token_metadata::{
            instructions::{
                CreateMasterEditionV3Cpi, 
                CreateMasterEditionV3CpiAccounts, 
                CreateMasterEditionV3InstructionArgs, 
                CreateMetadataAccountV3Cpi, 
                CreateMetadataAccountV3CpiAccounts, 
                CreateMetadataAccountV3InstructionArgs
            }, 
            types::{
                CollectionDetails, 
                Creator, 
                DataV2
            }
        }, 
        Metadata
    }, 
    token::{
        mint_to, 
        Mint, 
        MintTo, 
        Token, 
        TokenAccount
    }
};

use crate::state::Config;

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [b"collection", config.key().as_ref()],
        bump,
    )]
    pub collection: Account<'info, Mint>,
    /// CHECK: Collection NFT Metadata Account to be Initialized by Token Metadata Program
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
    pub collection_metadata: AccountInfo<'info>,
    /// CHECK: Collection NFT Master Edition Account to be Initialized by Token Metadata Program
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
    pub collection_edition: AccountInfo<'info>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = collection,
        associated_token::authority = config,
    )]
    pub collection_ata: Box<Account<'info, TokenAccount>>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    metadata_program: Program<'info, Metadata>,
}

impl<'info> CreateCollection<'info> {
    pub fn create_collection(&mut self, name: String, symbol: String, uri: String) -> Result<()> {

        // Accounts for the CPI calls
        let metadata = &self.collection_metadata.to_account_info();
        let master_edition = &self.collection_edition.to_account_info();
        let mint = &self.collection.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.authority.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let spl_token_program = &self.token_program.to_account_info();
        let spl_metadata_program = &self.metadata_program.to_account_info();

        // Signer seeds for CPI calls
        let seeds = &[
            &b"config"[..],
            &self.authority.key.as_ref(),
            &[self.config.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // CPI program and accounts for minting the Collection NFT
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.collection.to_account_info(),
            to: self.collection_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        // CPI Context for minting the Collection NFT
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Mint the Collection NFT
        mint_to(cpi_ctx, 1)?;

        // Create the creator array for the Collection NFT
        let creator = vec![
            Creator {
                address: self.config.key().clone(),
                verified: true,
                share: 100,
            },
        ];
        
        // Create the Collection NFT Metadata
        CreateMetadataAccountV3Cpi::new(
            spl_metadata_program, 
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: authority,
                payer,
                update_authority: (authority, true),
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: Some(
                    CollectionDetails::V1 { 
                        size: 0 
                    }
                )
            }
        ).invoke_signed(signer_seeds)?;

        // Create the Collection NFT Master Edition
        CreateMasterEditionV3Cpi::new(
            spl_metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition: master_edition,
                update_authority: authority,
                mint_authority: authority,
                mint,
                payer,
                metadata,
                token_program: spl_token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            }
        ).invoke_signed(signer_seeds)?;
        
        Ok(())
    }
}