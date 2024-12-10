use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
declare_id!("2GEjNvm8P1npWqX2ctzYtEkPpuJ5VFaDGQAQjdi9WiWF");
#[program]
pub mod create_token {
    use super::*;
    pub fn create_token_mint(_ctx: Context<CreateTokenMintContext>, _decimals: u8) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(_decimals:u8)]
pub struct CreateTokenMintContext<'info> {
    #[account(init, payer = payer, mint::decimals = _decimals, mint::authority = payer)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
