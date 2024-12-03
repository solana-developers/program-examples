use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}
};
declare_id!("7drtUeP5AWkcXTN9jLMA9zpDNyGT3FCgbX96yMvuxFrJ");
#[program]
pub mod spl_mint_program {
    use super::*;
    use anchor_spl::token_2022::{initialize_account3, mint_to, InitializeAccount3, MintTo};
    pub fn create(ctx: Context<CreateContext>) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeAccount3 {
                account: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.maker.to_account_info(),
            },
        );
        initialize_account3(cpi_ctx)?;
        Ok(())
    }
    pub fn mint(ctx: Context<MintContext>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.auth.to_account_info(),
            },
        );
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateContext<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account()]
    pub token_account: Account<'info, TokenAccount>,
    #[account()]
    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}
#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account()]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub auth: Signer<'info>,
    #[account()]
    pub to: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}
