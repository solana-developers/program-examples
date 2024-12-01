use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{TokenAccount, Mint, Transfer as TransferSPL, Token, transfer as transfer_spl},
};
declare_id!("4JTogV8LakXjvx49uoRmzowWJ6mxtaFBksFk3kNGUdKW");
#[program]
pub mod secure_token_escrow_program {
    use super::*;
    pub fn create_token_exchange_offer(
        ctx: Context<CreateTokenExchangeOfferContext>,
        offered_token_amount: u64,
        requested_token_amount: u64,
        escrow_identifier: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_state.auth_bump = ctx.bumps.escrow_authority;
        ctx.accounts.escrow_state.vault_bump = ctx.bumps.escrow_vault;
        ctx.accounts.escrow_state.escrow_bump = ctx.bumps.escrow_state;
        ctx.accounts.escrow_state.maker = ctx.accounts.maker.key();
        ctx.accounts.escrow_state.token_mint_a = ctx.accounts.offered_token_mint.key();
        ctx.accounts.escrow_state.token_mint_b = ctx.accounts.requested_token_mint.key();
        ctx.accounts.escrow_state.token_b_wanted_amount = requested_token_amount;
        ctx.accounts.escrow_state.id = escrow_identifier;
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.maker_offered_token_account.to_account_info(),
            to: ctx.accounts.escrow_vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, offered_token_amount)?;
        Ok(())
    }
    pub fn accept_token_exchange_offer(
        ctx: Context<AcceptTokenExchangeOfferContext>,
    ) -> Result<()> {
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.taker_offered_token_account.to_account_info(),
            to: ctx.accounts.maker_requested_token_account.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, ctx.accounts.escrow_state.token_b_wanted_amount)?;
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.taker_receive_token_account.to_account_info(),
            authority: ctx.accounts.escrow_authority.to_account_info(),
        };
        let signer_seeds = &[&b"auth"[..], &[ctx.accounts.escrow.auth_bump]];
        let binding = [&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &binding,
        );
        transfer_spl(cpi_ctx, ctx.accounts.escrow_state.token_b_wanted_amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateTokenExchangeOfferContext<'info> {
    #[account(
        init,
        payer = maker,
        space = 123,
        seeds = [b"escrow",
        maker.key().as_ref(),
        escrowIdentifier.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow_state: Account<'info, Escrow>,
    #[account()]
    pub requested_token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = maker,
        seeds = [b"vault",
        escrowState.key().as_ref()],
        token::mint = offered_token_mint,
        token::authority = escrow_authority,
        bump,
    )]
    pub escrow_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = offered_token_mint,
        associated_token::authority = maker,
    )]
    pub maker_offered_token_account: Account<'info, TokenAccount>,
    #[account()]
    pub offered_token_mint: Account<'info, Mint>,
    #[account(seeds = [b"auth"], bump)]
    /// CHECK: This acc is safe
    pub escrow_authority: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct AcceptTokenExchangeOfferContext<'info> {
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = offered_token_mint,
        associated_token::authority = taker,
    )]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"escrow",
        maker.key().as_ref(),
        escrowState.id.to_le_bytes().as_ref()],
        has_one = maker,
        has_one = offered_token_mint,
        has_one = requested_token_mint,
        bump,
        close = maker,
    )]
    pub escrow_state: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = offered_token_mint,
        associated_token::authority = maker,
    )]
    pub maker_requested_token_account: Account<'info, TokenAccount>,
    #[account()]
    pub requested_token_mint: Account<'info, Mint>,
    #[account()]
    pub offered_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = offered_token_mint,
        associated_token::authority = taker,
    )]
    pub taker_offered_token_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"auth"], bump)]
    /// CHECK: This acc is safe
    pub escrow_authority: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault",
        escrowState.key().as_ref()],
        token::mint = offered_token_mint,
        token::authority = escrow_authority,
        bump,
    )]
    pub escrow_vault: Account<'info, TokenAccount>,
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
