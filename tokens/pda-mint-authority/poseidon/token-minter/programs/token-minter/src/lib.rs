use anchor_lang::prelude::*;
use anchor_spl::{
    token::Mint,
    associated_token::AssociatedToken,
};
declare_id!("EWEURHBPCLgFnxMV6yKmmj2xS9386Rcr2ixBah8Pyjjv");
#[program]
pub mod token_minter {
    use super::*;
    pub fn mint_token(ctx: Context<MintTokenContext>, amount: u64) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct MintTokenContext<'info> {
    #[account()]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = maker_mint_account,
        associated_token::authority = auth,
        has_one = maker,
        has_one = maker_mint_account,
    )]
    pub maker_associated_token_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"mint"], bump)]
    pub maker_mint_account: Account<'info, Mint>,
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
