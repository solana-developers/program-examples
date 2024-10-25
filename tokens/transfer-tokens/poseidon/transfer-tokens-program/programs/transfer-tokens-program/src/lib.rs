use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Transfer as TransferSPL, Mint, transfer as transfer_spl, TokenAccount, Token},
};
declare_id!("BSHN8q3tEDsSiHBEHKKgxevQoSvXpKciZ7W3kcWSuEfC");
#[program]
pub mod transfer_tokens_program {
    use super::*;
    pub fn transfer_tokens(
        ctx: Context<TransferTokensContext>,
        transfer_amount: u64,
    ) -> Result<()> {
        let cpi_accounts = TransferSPL {
            from: ctx.accounts.source_ata.to_account_info(),
            to: ctx.accounts.destination_ata.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer_spl(cpi_ctx, transfer_amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct TransferTokensContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = destination,
    )]
    pub destination_ata: Account<'info, TokenAccount>,
    #[account()]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub source_ata: Account<'info, TokenAccount>,
    /// CHECK: Not necessary to enforce authority verification here
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
