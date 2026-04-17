use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::ErrorCode;
use crate::state::{
    add_ask, add_bid, add_open_order, Market, Order, OrderBook, OrderSide, OrderStatus,
    UserAccount, MAX_ORDERS_PER_SIDE, ORDER_BOOK_SEED, ORDER_SEED, USER_ACCOUNT_SEED,
};

// Mirror of UserAccount.open_orders max_len. Kept as a constant so the
// PlaceOrder check reads clearly and the limit is documented in one place.
const MAX_OPEN_ORDERS_PER_USER: usize = 20;

pub fn place_order(
    context: Context<PlaceOrderAccountConstraints>,
    side: OrderSide,
    price: u64,
    quantity: u64,
) -> Result<()> {
    let market = &context.accounts.market;

    require!(market.is_active, ErrorCode::MarketPaused);
    require!(price > 0, ErrorCode::InvalidPrice);
    require!(price % market.tick_size == 0, ErrorCode::InvalidTickSize);
    require!(
        quantity >= market.min_order_size,
        ErrorCode::BelowMinOrderSize
    );

    let order_book = &mut context.accounts.order_book;
    require!(
        order_book.bids.len() + order_book.asks.len() < MAX_ORDERS_PER_SIDE * 2,
        ErrorCode::OrderBookFull
    );

    let user_account = &mut context.accounts.user_account;
    require!(
        user_account.open_orders.len() < MAX_OPEN_ORDERS_PER_USER,
        ErrorCode::TooManyOpenOrders
    );

    let order_id = order_book.next_order_id;
    order_book.next_order_id = order_book.next_order_id.saturating_add(1);

    let order = &mut context.accounts.order;
    order.market = market.key();
    order.owner = context.accounts.owner.key();
    order.order_id = order_id;
    order.side = side;
    order.price = price;
    order.original_quantity = quantity;
    order.filled_quantity = 0;
    order.status = OrderStatus::Open;
    order.timestamp = Clock::get()?.unix_timestamp;
    order.bump = context.bumps.order;

    // Lock up the funds the order would need if filled. Bids lock quote
    // (price * quantity); asks lock base (quantity). Funds sit in the vault
    // until the order is cancelled (returned to unsettled) or settled.
    let (source_account, mint_account_info, decimals, transfer_amount, destination_vault) =
        match side {
            OrderSide::Bid => (
                context.accounts.user_quote_account.to_account_info(),
                context.accounts.quote_mint.to_account_info(),
                context.accounts.quote_mint.decimals,
                price
                    .checked_mul(quantity)
                    .ok_or(ErrorCode::NumericalOverflow)?,
                context.accounts.quote_vault.to_account_info(),
            ),
            OrderSide::Ask => (
                context.accounts.user_base_account.to_account_info(),
                context.accounts.base_mint.to_account_info(),
                context.accounts.base_mint.decimals,
                quantity,
                context.accounts.base_vault.to_account_info(),
            ),
        };

    transfer_checked(
        CpiContext::new(
            context.accounts.token_program.key(),
            TransferChecked {
                from: source_account,
                mint: mint_account_info,
                to: destination_vault,
                authority: context.accounts.owner.to_account_info(),
            },
        ),
        transfer_amount,
        decimals,
    )?;

    match side {
        OrderSide::Bid => add_bid(order_book, order_id, price, context.accounts.owner.key()),
        OrderSide::Ask => add_ask(order_book, order_id, price, context.accounts.owner.key()),
    }

    add_open_order(user_account, order_id);

    Ok(())
}

#[derive(Accounts)]
#[instruction(side: OrderSide, price: u64, quantity: u64)]
pub struct PlaceOrderAccountConstraints<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [ORDER_BOOK_SEED, market.key().as_ref()],
        bump = order_book.bump
    )]
    pub order_book: Account<'info, OrderBook>,

    #[account(
        init,
        payer = owner,
        space = Order::DISCRIMINATOR.len() + Order::INIT_SPACE,
        seeds = [
            ORDER_SEED,
            market.key().as_ref(),
            order_book.next_order_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub order: Account<'info, Order>,

    #[account(
        mut,
        seeds = [USER_ACCOUNT_SEED, market.key().as_ref(), owner.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    // InterfaceAccount on the stack is ~1 KB each; with 6 of them this struct
    // blows the 4 KB stack-offset limit on BPF. Boxing moves each to the heap.
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

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
