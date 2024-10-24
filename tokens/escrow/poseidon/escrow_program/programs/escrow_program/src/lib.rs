use anchor_lang::prelude::*;
use anchor_spl::{
    token::{
        Mint, TokenAccount, Token, transfer as transfer_spl, Transfer as TransferSPL,
    },
    associated_token::AssociatedToken,
};
declare_id!("8b7Pshe6L28ee9oCTjGHCYTugFCQjKSBGfbaXZt9f3EF");
#[program]
pub mod escrow_program {
    use super::*;
    pub fn make_offer(
        ctx: Context<MakeOfferContext>,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
        id: u64,
    ) -> Result<()> {
        ctx.accounts.escrow.auth_bump = ctx.bumps.auth;
        ctx.accounts.escrow.vault_bump = ctx.bumps.vault;
        ctx.accounts.escrow.escrow_bump = ctx.bumps.escrow;
        ctx.accounts.escrow.maker = ctx.accounts.maker.key();
        ctx.accounts.escrow.token_mint_a = ctx.accounts.token_mint_a.key();
        ctx.accounts.escrow.token_mint_b = ctx.accounts.token_mint_b.key();
        ctx.accounts.escrow.token_b_wanted_amount = token_b_wanted_amount;
        ctx.accounts.escrow.id = id;
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.maker_token_account_a.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, token_a_offered_amount)?;
        Ok(())
    }
    pub fn take_offer(ctx: Context<TakeOfferContext>) -> Result<()> {
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.taker_token_account_a.to_account_info(),
            to: ctx.accounts.maker_token_account_a.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, ctx.accounts.escrow.token_b_wanted_amount)?;
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.taker_token_account_b.to_account_info(),
            authority: ctx.accounts.auth.to_account_info(),
        };
        let signer_seeds = &[&b"auth"[..], &[ctx.accounts.escrow.auth_bump]];
        let binding = [&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &binding,
        );
        transfer_spl(cpi_ctx, ctx.accounts.escrow.token_b_wanted_amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(id:u64)]
pub struct MakeOfferContext<'info> {
    #[account(
        init,
        payer = maker,
        space = 123,
        seeds = [b"escrow",
        maker.key().as_ref(),
        id.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
    )]
    pub maker_token_account_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account()]
    pub token_mint_a: Account<'info, Mint>,
    #[account()]
    pub token_mint_b: Account<'info, Mint>,
    #[account(
        init,
        payer = maker,
        seeds = [b"vault",
        escrow.key().as_ref()],
        token::mint = token_mint_a,
        token::authority = auth,
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(seeds = [b"auth"], bump)]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TakeOfferContext<'info> {
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault",
        escrow.key().as_ref()],
        token::mint = token_mint_a,
        token::authority = auth,
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account()]
    pub token_mint_b: Account<'info, Mint>,
    #[account(seeds = [b"auth"], bump)]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"escrow",
        maker.key().as_ref(),
        escrow.id.to_le_bytes().as_ref()],
        has_one = maker,
        has_one = token_mint_a,
        has_one = token_mint_b,
        bump,
        close = maker,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
    )]
    pub taker_token_account_a: Account<'info, TokenAccount>,
    #[account()]
    pub token_mint_a: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
    )]
    pub maker_token_account_a: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
    )]
    pub taker_token_account_b: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>, 
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub escrow_bump: u8,
    pub id: u64,
    pub auth_bump: u8,
    pub vault_bump: u8,
}
