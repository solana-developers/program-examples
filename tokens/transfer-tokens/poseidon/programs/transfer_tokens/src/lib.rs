use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        TokenAccount, mint_to, Transfer as TransferSPL, Token, MintTo,
        transfer as transfer_spl, Mint,
    },
    metadata::{
        CreateMetadataAccountsV3, mpl_token_metadata, create_metadata_accounts_v3,
        Metadata,
    },
};
declare_id!("4h2WWD9id7t75bNDwwWRoWYh759MDePhPZFiFJat9E9S");
#[program]
pub mod transfer_tokens_program {
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
    pub fn mint_token(ctx: Context<MintTokenContext>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        );
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
    pub fn transfer_tokens(
        ctx: Context<TransferTokensContext>,
        amount: u64,
    ) -> Result<()> {
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(token_decimals:u8)]
pub struct CreateTokenMintContext<'info> {
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
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(Accounts)]
pub struct MintTokenContext<'info> {
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = mint_authority,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TransferTokensContext<'info> {
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
