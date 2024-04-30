use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    withdraw_withheld_tokens_from_mint, Mint, Token2022, TokenAccount,
    WithdrawWithheldTokensFromMint,
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
}

// transfer fees "harvested" to the mint account can then be withdraw by the withdraw authority
// this transfers fees on the mint account to the specified token account
pub fn process_withdraw(ctx: Context<Withdraw>) -> Result<()> {
    withdraw_withheld_tokens_from_mint(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        WithdrawWithheldTokensFromMint {
            token_program_id: ctx.accounts.token_program.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            destination: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    ))?;
    Ok(())
}
