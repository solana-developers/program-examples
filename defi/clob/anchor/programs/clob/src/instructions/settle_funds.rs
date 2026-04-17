use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::state::{Market, UserAccount, MARKET_SEED, USER_ACCOUNT_SEED};

pub fn settle_funds(context: Context<SettleFundsAccountConstraints>) -> Result<()> {
    let user_account = &mut context.accounts.user_account;
    let market = &context.accounts.market;

    let base_amount = user_account.unsettled_base;
    let quote_amount = user_account.unsettled_quote;

    // Seeds to sign as the market PDA (the authority of both vaults). Built
    // once and reused for the two possible transfers.
    let market_bump = [market.bump];
    let signer_seeds: [&[u8]; 4] = [
        MARKET_SEED,
        market.base_mint.as_ref(),
        market.quote_mint.as_ref(),
        &market_bump,
    ];
    let signer_seeds = &[&signer_seeds[..]];

    if base_amount > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                context.accounts.token_program.key(),
                TransferChecked {
                    from: context.accounts.base_vault.to_account_info(),
                    mint: context.accounts.base_mint.to_account_info(),
                    to: context.accounts.user_base_account.to_account_info(),
                    authority: market.to_account_info(),
                },
                signer_seeds,
            ),
            base_amount,
            context.accounts.base_mint.decimals,
        )?;
        user_account.unsettled_base = 0;
    }

    if quote_amount > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                context.accounts.token_program.key(),
                TransferChecked {
                    from: context.accounts.quote_vault.to_account_info(),
                    mint: context.accounts.quote_mint.to_account_info(),
                    to: context.accounts.user_quote_account.to_account_info(),
                    authority: market.to_account_info(),
                },
                signer_seeds,
            ),
            quote_amount,
            context.accounts.quote_mint.decimals,
        )?;
        user_account.unsettled_quote = 0;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct SettleFundsAccountConstraints<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [USER_ACCOUNT_SEED, market.key().as_ref(), owner.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    // Boxed for the same reason as in PlaceOrderAccountConstraints —
    // InterfaceAccount is too large to keep on the BPF stack in bulk.
    #[account(mut)]
    pub base_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user_base_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user_quote_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub base_mint: Box<InterfaceAccount<'info, Mint>>,

    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    pub owner: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}
