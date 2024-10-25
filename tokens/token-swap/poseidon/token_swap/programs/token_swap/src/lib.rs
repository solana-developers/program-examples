use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer as transfer_spl, Mint, Transfer as TransferSPL},
    associated_token::AssociatedToken,
};
declare_id!("EvcknV23Y3dkbSa4afZNGw2PgoowcfxCy4qvP8Ghogwu");
#[program]
pub mod token_swap {
    use super::*;
    pub fn create_amm(
        ctx: Context<CreateAmmContext>,
        id: Pubkey,
        fee: u16,
    ) -> Result<()> {
        ctx.accounts.amm.id = id;
        ctx.accounts.amm.admin = ctx.accounts.admin.key();
        ctx.accounts.amm.fee = fee;
        Ok(())
    }
    pub fn create_pool(
        ctx: Context<CreatePoolContext>,
        id: Pubkey,
        fee: u16,
    ) -> Result<()> {
        Ok(())
    }
    pub fn deposit_liquidity(
        ctx: Context<DepositLiquidityContext>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.depositor_account_a.to_account_info(),
            to: ctx.accounts.pool_account_a.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, amount_a)?;
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.depositor_account_b.to_account_info(),
            to: ctx.accounts.pool_account_b.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, amount_b)?;
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_liquidity.to_account_info(),
                to: ctx.accounts.depositor_account_liquidity.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            signer,
        );
        mint_to(cpi_ctx, liquidity)?;
        Ok(())
    }
    pub fn swap_exact_tokens_for_tokens(
        ctx: Context<SwapExactTokensForTokensContext>,
        amount_a: u64,
        amount_b: u64,
        input_amount: u64,
        min_input_amount: u64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn withdraw_liquidity(
        ctx: Context<WithdrawLiquidityContext>,
        amount: u64,
    ) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(id:Pubkey)]
pub struct CreateAmmContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub admin: Account<'info, Admin>,
    #[account(
        init,
        payer = payer,
        space = 74,
        seeds = [id.to_le_bytes().as_ref()],
        bump,
    )]
    pub amm: Account<'info, AMM>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(id:Pubkey)]
pub struct CreatePoolContext<'info> {
    #[account(
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref(),
        b"authority"],
        bump,
    )]
    pub pool_authority: Account<'info, poolAuthority>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Account<'info, TokenAccount>,
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
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref(),
        b"liquidity"],
        bump,
    )]
    pub mint_liquidity: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        space = 74,
        seeds = [id.to_le_bytes().as_ref()],
        bump,
    )]
    pub amm: Account<'info, AMM>,
    #[account()]
    pub mint_b: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 104,
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DepositLiquidityContext<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
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
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref(),
        b"liquidity"],
        bump,
    )]
    pub mint_liquidity: Account<'info, Mint>,
    #[account()]
    pub amm: Account<'info, AMM>,
    #[account(
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref(),
        b"authority"],
        bump,
    )]
    pub pool_authority: Account<'info, poolAuthority>,
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
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    pub depositor_account_a: Account<'info, TokenAccount>,
    #[account()]
    pub mint_a: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_b: Account<'info, TokenAccount>,
    #[account()]
    pub mint_b: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SwapExactTokensForTokensContext<'info> {
    #[account(
        init,
        payer = trader,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_b: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = trader,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Account<'info, TokenAccount>,
    #[account()]
    pub mint_b: Account<'info, Mint>,
    #[account(mut)]
    pub trader: Signer<'info>,
    #[account(
        init,
        payer = trader,
        associated_token::mint = mint_a,
        associated_token::authority = trader,
    )]
    pub trader_account_a: Account<'info, TokenAccount>,
    #[account(seeds = [amm.id.to_le_bytes().as_ref()], bump)]
    pub amm: Account<'info, AMM>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub mint_a: Account<'info, Mint>,
    #[account(
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref(),
        b"authority"],
        bump,
    )]
    pub pool_authority: Account<'info, poolAuthority>,
    #[account(
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        init,
        payer = trader,
        associated_token::mint = mint_b,
        associated_token::authority = trader,
    )]
    pub trader_account_b: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct WithdrawLiquidityContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref(),
        b"authority"],
        bump,
    )]
    pub pool_authority: Account<'info, poolAuthority>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_b,
        associated_token::authority = depositor,
    )]
    pub depositor_account_b: Account<'info, TokenAccount>,
    #[account(seeds = [amm.id.to_le_bytes().as_ref()], bump)]
    pub amm: Account<'info, AMM>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    pub depositor_account_a: Account<'info, TokenAccount>,
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
    #[account()]
    pub mint_a: Account<'info, Mint>,
    #[account()]
    pub mint_liquidity: Account<'info, Mint>,
    #[account(
        seeds = [amm.key.to_le_bytes().as_ref(),
        mint_a.key.to_le_bytes().as_ref(),
        mint_b.key.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account()]
    pub mint_b: Account<'info, Mint>,
    #[account(
        init,
        payer = depositor,
        associated_token::mint = mint_liquidity,
        associated_token::authority = depositor,
    )]
    pub depositor_account_liquidity: Account<'info, TokenAccount>,
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
pub struct Admin {}
#[account]
pub struct AMM {
    pub id: Pubkey,
    pub admin: Pubkey,
    pub fee: u16,
}
#[account]
pub struct poolAuthority {
    pub pool_authority_bump: u8,
}
