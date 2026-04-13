use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{transfer_checked, TransferChecked},
    token_interface::{Mint, Token2022, TokenAccount},
};

// Note that you cannot initialize or update the CpiGuard extension through a CPI
// https://github.com/solana-labs/solana-program-library/blob/6968859e2ee0a1764da572de340cdb58e2b4586f/token/program-2022/src/extension/cpi_guard/processor.rs#L44-L46
declare_id!("6tU3MEowU6oxxeDZLSxEwzcEZsZrhBJsfUR6xECvShid");

#[program]
pub mod cpi_guard {
    use super::*;

    pub fn cpi_transfer(context: Context<CpiTransfer>) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                context.accounts.token_program.key(),
                TransferChecked {
                    from: context.accounts.sender_token_account.to_account_info(),
                    mint: context.accounts.mint_account.to_account_info(),
                    to: context.accounts.recipient_token_account.to_account_info(),
                    authority: context.accounts.sender.to_account_info(),
                },
            ),
            1,
            context.accounts.mint_account.decimals,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        token::mint = mint_account
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        seeds = [b"pda"],
        bump,
        token::mint = mint_account,
        token::authority = recipient_token_account,
        token::token_program = token_program
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
