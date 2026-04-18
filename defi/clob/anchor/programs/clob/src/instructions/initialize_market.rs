use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::ErrorCode;
use crate::state::{Market, OrderBook, MARKET_SEED, ORDER_BOOK_SEED};

// Basis-points are hundredths of a percent; 10000 bps == 100%. Fees above 100%
// would be nonsensical, so we cap here.
const MAX_FEE_BASIS_POINTS: u16 = 10_000;

pub fn handle_initialize_market(
    context: Context<InitializeMarket>,
    fee_basis_points: u16,
    tick_size: u64,
    min_order_size: u64,
) -> Result<()> {
    require!(tick_size > 0, ErrorCode::InvalidTickSize);
    require!(min_order_size > 0, ErrorCode::BelowMinOrderSize);
    require!(
        fee_basis_points <= MAX_FEE_BASIS_POINTS,
        ErrorCode::InvalidFeeBasisPoints
    );

    let market = &mut context.accounts.market;
    market.authority = context.accounts.authority.key();
    market.base_mint = context.accounts.base_mint.key();
    market.quote_mint = context.accounts.quote_mint.key();
    market.base_vault = context.accounts.base_vault.key();
    market.quote_vault = context.accounts.quote_vault.key();
    market.order_book = context.accounts.order_book.key();
    market.fee_basis_points = fee_basis_points;
    market.tick_size = tick_size;
    market.min_order_size = min_order_size;
    market.is_active = true;
    market.bump = context.bumps.market;

    let order_book = &mut context.accounts.order_book;
    order_book.market = context.accounts.market.key();
    order_book.bids = Vec::new();
    order_book.asks = Vec::new();
    // Start at 1 so order_id == 0 can stand for "no order" in clients if needed.
    order_book.next_order_id = 1;
    order_book.bump = context.bumps.order_book;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        payer = authority,
        space = Market::DISCRIMINATOR.len() + Market::INIT_SPACE,
        seeds = [MARKET_SEED, base_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = authority,
        space = OrderBook::DISCRIMINATOR.len() + OrderBook::INIT_SPACE,
        seeds = [ORDER_BOOK_SEED, market.key().as_ref()],
        bump
    )]
    pub order_book: Account<'info, OrderBook>,

    pub base_mint: InterfaceAccount<'info, Mint>,

    pub quote_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = base_mint,
        token::authority = market,
        token::token_program = token_program
    )]
    pub base_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = quote_mint,
        token::authority = market,
        token::token_program = token_program
    )]
    pub quote_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
