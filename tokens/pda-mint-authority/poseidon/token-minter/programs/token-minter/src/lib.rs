use anchor_lang::prelude::*;
use anchor_spl::{
    token::{mint_to, Token, Mint, TokenAccount, MintTo},
    associated_token::AssociatedToken,
};
declare_id!("5jVxRAH6W8C8SNdX3HUnabC1r3F9MxnNHfKTBe2DRXkT");
#[program]
pub mod token_minter {
    use super::*;
    pub fn create_token(
        ctx: Context<CreateTokenContext>,
        decimals: u8,
        freeze_authority: Pubkey,
    ) -> Result<()> {
        Ok(())
    }
    pub fn mint_token(ctx: Context<MintTokenContext>, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]; 1] = &[&[b"mint", &[ctx.bumps.mint_account]]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(),
            },
            signer_seeds,
        );
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateTokenContext<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut, seeds = [b"mint"], bump)]
    pub mint_account: Account<'info, Mint>,
}
#[derive(Accounts)]
pub struct MintTokenContext<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut, seeds = [b"mint"], bump)]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
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
