use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3,
            Metadata,
        },
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
    },
};

declare_id!("52quezNUzc1Ej6Jh6L4bvtxPW8j6TEFHuLVAWiFvdnsc");

#[program]
pub mod nft_minter {
    use super::*;

    pub fn mint_nft(
        context: Context<CreateTokenAccountConstraints>,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        msg!("Minting Token");
        // Cross Program Invocation (CPI)
        // Invoking the mint_to instruction on the token program
        mint_to(
            CpiContext::new(
                context.accounts.token_program.key(),
                MintTo {
                    mint: context.accounts.mint_account.to_account_info(),
                    to: context.accounts.associated_token_account.to_account_info(),
                    authority: context.accounts.payer.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("Creating metadata account");
        // Cross Program Invocation (CPI)
        // Invoking the create_metadata_account_v3 instruction on the token metadata program
        create_metadata_accounts_v3(
            CpiContext::new(
                context.accounts.token_metadata_program.key(),
                CreateMetadataAccountsV3 {
                    metadata: context.accounts.metadata_account.to_account_info(),
                    mint: context.accounts.mint_account.to_account_info(),
                    mint_authority: context.accounts.payer.to_account_info(),
                    update_authority: context.accounts.payer.to_account_info(),
                    payer: context.accounts.payer.to_account_info(),
                    system_program: context.accounts.system_program.to_account_info(),
                    rent: context.accounts.rent.to_account_info(),
                },
            ),
            DataV2 {
                name: nft_name,
                symbol: nft_symbol,
                uri: nft_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            false, // Is mutable
            true,  // Update authority is signer
            None,  // Collection details
        )?;

        msg!("Creating master edition account");
        // Cross Program Invocation (CPI)
        // Invoking the create_master_edition_v3 instruction on the token metadata program
        create_master_edition_v3(
            CpiContext::new(
                context.accounts.token_metadata_program.key(),
                CreateMasterEditionV3 {
                    edition: context.accounts.edition_account.to_account_info(),
                    mint: context.accounts.mint_account.to_account_info(),
                    update_authority: context.accounts.payer.to_account_info(),
                    mint_authority: context.accounts.payer.to_account_info(),
                    payer: context.accounts.payer.to_account_info(),
                    metadata: context.accounts.metadata_account.to_account_info(),
                    token_program: context.accounts.token_program.to_account_info(),
                    system_program: context.accounts.system_program.to_account_info(),
                    rent: context.accounts.rent.to_account_info(),
                },
            ),
            None, // Max Supply
        )?;

        msg!("NFT minted successfully.");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTokenAccountConstraints<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    // Create new mint account, NFTs have 0 decimals
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
