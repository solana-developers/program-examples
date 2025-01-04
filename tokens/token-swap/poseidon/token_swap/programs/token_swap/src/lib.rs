use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken,
};
declare_id!("3dDaJxmPcmQVfSx9rX4xHyP5rJvkwdKcNujcX2z9KB9h");
#[program]
pub mod token_swap {
    use super::*;
    pub fn create_amm(ctx: Context<CreateAmmContext>, id: u64, fee: u16) -> Result<()> {
        ctx.accounts.amm.id = id;
        ctx.accounts.amm.admin = ctx.accounts.admin.key();
        ctx.accounts.amm.fee = fee;
        Ok(())
    }
    pub fn create_pool(ctx: Context<CreatePoolContext>, id: u64) -> Result<()> {
        ctx.accounts.pool.amm = ctx.accounts.amm.key();
        ctx.accounts.pool.mint_a = ctx.accounts.mint_a.key();
        ctx.accounts.pool.mint_b = ctx.accounts.mint_b.key();
        Ok(())
    }
    pub fn deposit_liquidity(
        ctx: Context<DepositLiquidityContext>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn swap_exact_tokens_for_tokens(
        ctx: Context<SwapExactTokensForTokensContext>,
        fee: u16,
        amount_a: u64,
        amount_b: u64,
        input_amount: u64,
        min_input_amount: u64,
        id: u64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn withdraw_liquidity(
        ctx: Context<WithdrawLiquidityContext>,
        amount: u64,
        id: u64,
    ) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(id:u64)]
pub struct CreateAmmContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 8, seeds = [b"admin"], bump)]
    pub admin: Account<'info, Admin>,
    #[account(
        init,
        payer = payer,
        space = 50,
        seeds = [id.to_le_bytes().as_ref()],
        bump,
    )]
    pub amm: Account<'info, AMM>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(id:u64)]
pub struct CreatePoolContext<'info> {
    #[account(
        init,
        payer = payer,
        space = 104,
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Account<'info, TokenAccount>,
    #[account(
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref(),
        b"authority"],
        bump,
    )]
    pub pool_authority: Account<'info, PoolAuthority>,
    #[account(
        init,
        payer = payer,
        space = 82,
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref(),
        b"liquidity"],
        bump,
    )]
    pub mint_liquidity: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_b: Account<'info, TokenAccount>,
    #[account()]
    pub mint_a: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        space = 50,
        seeds = [id.to_le_bytes().as_ref()],
        bump,
    )]
    pub amm: Account<'info, AMM>,
    #[account()]
    pub mint_b: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DepositLiquidityContext<'info> {
    #[account(
        init,
        payer = depositor,
        space = 82,
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref(),
        b"liquidity"],
        bump,
    )]
    pub mint_liquidity: Account<'info, Mint>,
    #[account()]
    pub mint_a: Account<'info, Mint>,
    #[account()]
    pub mint_b: Account<'info, Mint>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    pub depositor_account_a: Account<'info, TokenAccount>,
    #[account(
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account()]
    pub amm: Account<'info, AMM>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_b: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_b,
        associated_token::authority = depositor,
    )]
    pub depositor_account_b: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_liquidity,
        associated_token::authority = depositor,
    )]
    pub depositor_account_liquidity: Account<'info, TokenAccount>,
    #[account(
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref(),
        b"authority"],
        bump,
    )]
    pub pool_authority: Account<'info, PoolAuthority>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(id:u64)]
pub struct SwapExactTokensForTokensContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub mint_b: Account<'info, Mint>,
    #[account(
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref(),
        b"authority"],
        bump,
    )]
    pub pool_authority: Account<'info, PoolAuthority>,
    #[account(
        init,
        payer = trader,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = trader,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_b: Account<'info, TokenAccount>,
    #[account(seeds = [id.to_le_bytes().as_ref()], bump)]
    pub amm: Account<'info, AMM>,
    #[account(
        init,
        payer = trader,
        associated_token::mint = mint_a,
        associated_token::authority = trader,
    )]
    pub trader_account_a: Account<'info, TokenAccount>,
    #[account(
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub trader: Signer<'info>,
    #[account(
        init,
        payer = trader,
        associated_token::mint = mint_b,
        associated_token::authority = trader,
    )]
    pub trader_account_b: Account<'info, TokenAccount>,
    #[account()]
    pub mint_a: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(id:u64)]
pub struct WithdrawLiquidityContext<'info> {
    #[account(seeds = [id.to_le_bytes().as_ref()], bump)]
    pub amm: Account<'info, AMM>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account()]
    pub mint_liquidity: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_b,
        associated_token::authority = depositor,
    )]
    pub depositor_account_b: Account<'info, TokenAccount>,
    #[account()]
    pub mint_a: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    pub depositor_account_a: Account<'info, TokenAccount>,
    #[account()]
    pub mint_b: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_liquidity,
        associated_token::authority = depositor,
    )]
    pub depositor_account_liquidity: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_b: Account<'info, TokenAccount>,
    #[account(
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        seeds = [amm.key().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref(),
        b"authority"],
        bump,
    )]
    pub pool_authority: Account<'info, PoolAuthority>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Pool {
    pub amm: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
}
#[account]
pub struct PoolAuthority {}
#[account]
pub struct Admin {}
#[account]
pub struct AMM {
    pub id: u64,
    pub admin: Pubkey,
    pub fee: u16,
}
