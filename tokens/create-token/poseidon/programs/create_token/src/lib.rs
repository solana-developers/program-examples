use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token},
    metadata::{
        CreateMetadataAccountsV3, create_metadata_accounts_v3, Metadata,
        mpl_token_metadata,
    },
};
declare_id!("6xsCBfTAhmH8JQ6cXN69cvWaiDUG723k76MDMjt9fRah");
#[program]
pub mod create_token {
    use super::*;
    pub fn create_token_mint(
        ctx: Context<CreateTokenMintContext>,
        key: Pubkey,
        token_decimals: u8,
        token_name: String,
        token_symbol: String,
        token_uri: String,
        seller_fee_basis_points: u16,
    ) -> Result<()> {
        let token_data: mpl_token_metadata::types::DataV2 = mpl_token_metadata::types::DataV2 {
            name: token_name,
            symbol: token_symbol,
            uri: token_uri,
            seller_fee_basis_points: seller_fee_basis_points,
            creators: None,
            collection: None,
            uses: None,
        };
        let cpi_accounts = CreateMetadataAccountsV3 {
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            metadata: ctx.accounts.metadata_account.to_account_info(),
            mint_authority: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            cpi_accounts,
        );
        create_metadata_accounts_v3(cpi_ctx, token_data, true, false, None)?;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(token_decimals:u8)]
pub struct CreateTokenMintContext<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = token_decimals,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata",
        token_metadata_program.key().as_ref(),
        mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}
