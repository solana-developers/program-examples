use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer as transfer_spl, Transfer as TransferSPL, mint_to, Mint, MintTo, Token, TokenAccount},
    associated_token::AssociatedToken,
};
declare_id!("HFKNWrbYAfKsrWJu88RtUVHgVBNz1uJ6u2tNx1YCmAMZ");
#[program]
pub mod spl_token_minter {
    use super::*;
    pub fn create_token(
        ctx: Context<CreateTokenContext>,
        decimals: u8,
        freeze_authority: Pubkey,
    ) -> Result<()> {
        Ok(())
    }
    pub fn mint(ctx: Context<MintContext>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.to_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            }
        );
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateTokenContext<'info> {
    #[account()]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
}
#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}