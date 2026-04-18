use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::ErrorCode;
use crate::state::{Market, MARKET_SEED};

/// Drain the market's accumulated taker fees into the authority's token
/// account. Authority-only — arbitrary callers must not be able to siphon
/// the fee vault. Transfers the current balance of the fee vault in full;
/// a partial-withdraw flavour could take an amount parameter, left out here
/// to keep the example focused.
pub fn handle_withdraw_fees(context: Context<WithdrawFees>) -> Result<()> {
    let market = &context.accounts.market;

    require!(
        context.accounts.authority.key() == market.authority,
        ErrorCode::NotMarketAuthority
    );

    let fee_balance = context.accounts.fee_vault.amount;
    if fee_balance == 0 {
        // Nothing to do — exit quietly rather than failing, so this
        // instruction is safe to call on a cron/heartbeat even when there
        // haven't been any fills since the last run.
        return Ok(());
    }

    let market_bump = [market.bump];
    let signer_seeds: [&[u8]; 4] = [
        MARKET_SEED,
        market.base_mint.as_ref(),
        market.quote_mint.as_ref(),
        &market_bump,
    ];
    let signer_seeds = &[&signer_seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            context.accounts.token_program.key(),
            TransferChecked {
                from: context.accounts.fee_vault.to_account_info(),
                mint: context.accounts.quote_mint.to_account_info(),
                to: context.accounts.authority_quote_account.to_account_info(),
                authority: market.to_account_info(),
            },
            signer_seeds,
        ),
        fee_balance,
        context.accounts.quote_mint.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        mut,
        has_one = fee_vault @ ErrorCode::InvalidFeeVault,
    )]
    pub market: Account<'info, Market>,

    // Boxed to keep the struct under the BPF stack limit (see PlaceOrder).
    #[account(mut)]
    pub fee_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub authority_quote_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    pub authority: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}
