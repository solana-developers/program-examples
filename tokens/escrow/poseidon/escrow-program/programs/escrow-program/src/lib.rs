use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Transfer as TransferSPL, Token, transfer as transfer_spl, TokenAccount, Mint},
};
declare_id!("BCsW6bJJZPgLYba52dNie4L2SebE7ALHgQyWakLehvSz");
#[program]
pub mod escrow_program {
    use super::*;
    pub fn make(
        ctx: Context<MakeContext>,
        deposit_amount: u64,
        offer_amount: u64,
        seed: u64,
    ) -> Result<()> {
        ctx.accounts.escrow.auth_bump = ctx.bumps.auth;
        ctx.accounts.escrow.vault_bump = ctx.bumps.vault;
        ctx.accounts.escrow.escrow_bump = ctx.bumps.escrow;
        ctx.accounts.escrow.maker = ctx.accounts.maker.key();
        ctx.accounts.escrow.amount = offer_amount;
        ctx.accounts.escrow.seed = seed;
        ctx.accounts.escrow.maker_mint = ctx.accounts.maker_mint.key();
        ctx.accounts.escrow.taker_mint = ctx.accounts.taker_mint.key();
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.maker_ata.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, deposit_amount)?;
        Ok(())
    }
    pub fn refund(ctx: Context<RefundContext>) -> Result<()> {
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.maker_ata.to_account_info(),
            authority: ctx.accounts.auth.to_account_info(),
        };
        let signer_seeds = &[&b"auth"[..], &[ctx.accounts.escrow.auth_bump]];
        let binding = [&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &binding,
        );
        transfer_spl(cpi_ctx, ctx.accounts.escrow.amount)?;
        Ok(())
    }
    pub fn take(ctx: Context<TakeContext>) -> Result<()> {
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.taker_ata.to_account_info(),
            to: ctx.accounts.maker_ata.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, ctx.accounts.escrow.amount)?;
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.taker_receive_ata.to_account_info(),
            authority: ctx.accounts.auth.to_account_info(),
        };
        let signer_seeds = &[&b"auth"[..], &[ctx.accounts.escrow.auth_bump]];
        let binding = [&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &binding,
        );
        transfer_spl(cpi_ctx, ctx.accounts.escrow.amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(seed:u64)]    
pub struct MakeContext<'info> {
    #[account()]
    pub taker_mint: Account<'info, Mint>,
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(seeds = [b"auth"], bump)]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account()]
    pub maker_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = maker,
        space = 123,
        seeds = [b"escrow",
        maker.key().as_ref(),
        seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account(
        init,
        payer = maker,
        seeds = [b"vault",
        escrow.key().as_ref()],
        token::mint = maker_mint,
        token::authority = auth,
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RefundContext<'info> {
    #[account()]
    pub maker_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(seeds = [b"auth"], bump)]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault",
        escrow.key().as_ref()],
        token::mint = maker_mint,
        token::authority = auth,
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"escrow",
        maker.key().as_ref(),
        escrow.seed.to_le_bytes().as_ref()],
        has_one = maker,
        bump,
        close = maker,
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account(mut)]
    pub maker: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TakeContext<'info> {
    #[account(seeds = [b"auth"], bump)]
    /// CHECK: This acc is safe
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"escrow",
        maker.key().as_ref(),
        escrow.seed.to_le_bytes().as_ref()],
        has_one = maker,
        has_one = maker_mint,
        has_one = taker_mint,
        bump,
        close = maker,
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account()]
    pub maker_mint: Account<'info, Mint>,
    #[account()]
    pub taker_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault",
        escrow.key().as_ref()],
        token::mint = maker_mint,
        token::authority = auth,
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_receive_ata: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct EscrowState {
    pub maker: Pubkey,
    pub maker_mint: Pubkey,
    pub taker_mint: Pubkey,
    pub amount: u64,
    pub seed: u64,
    pub auth_bump: u8,
    pub escrow_bump: u8,
    pub vault_bump: u8,
}
