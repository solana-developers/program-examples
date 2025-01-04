use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenAccount, Token},
    associated_token::AssociatedToken,
};
declare_id!("AMXNdYTyDpcLLJ9CzVJQ1kw5gqE4JeZxjtUbH2MwntdD");
#[program]
pub mod token_minter {
    use super::*;
    pub fn create_token(
        ctx: Context<CreateTokenContext>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        Ok(())
    }
    pub fn mint(ctx: Context<MintContext>) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateTokenContext<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account()]
    pub maker_associated_token_account: Account<'info, TokenAccount>,
    #[account()]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account()]
    pub maker_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}
#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account(seeds = [b"mint"], bump)]
    pub maker_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = maker_mint,
        associated_token::authority = payer,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct MintToken {
    pub maker: Pubkey,
    pub maker_mint_account: Pubkey,
    pub maker_mint_bump: u8,
    pub seed: u64,
    pub auth_bump: u8,
    pub amount: u64,
}
#[account]
pub struct CreateToken {}
