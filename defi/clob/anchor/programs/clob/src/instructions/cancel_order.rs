use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::{
    remaining_quantity, remove_open_order, remove_order, Market, Order, OrderBook, OrderSide,
    OrderStatus, UserAccount, ORDER_BOOK_SEED, ORDER_SEED, USER_ACCOUNT_SEED,
};

pub fn handle_cancel_order(context: Context<CancelOrder>) -> Result<()> {
    let order = &mut context.accounts.order;

    require!(
        order.owner == context.accounts.owner.key(),
        ErrorCode::Unauthorized
    );

    require!(
        order.status == OrderStatus::Open || order.status == OrderStatus::PartiallyFilled,
        ErrorCode::OrderNotCancellable
    );

    // Funds the order had locked in the vault are now owed back to the
    // owner. Credit the appropriate unsettled balance; settle_funds moves
    // those funds from the vault to the owner's token account.
    let remaining = remaining_quantity(order);
    if remaining > 0 {
        let user_account = &mut context.accounts.user_account;
        match order.side {
            OrderSide::Bid => {
                let quote_amount = order
                    .price
                    .checked_mul(remaining)
                    .ok_or(ErrorCode::NumericalOverflow)?;
                user_account.unsettled_quote = user_account
                    .unsettled_quote
                    .checked_add(quote_amount)
                    .ok_or(ErrorCode::NumericalOverflow)?;
            }
            OrderSide::Ask => {
                user_account.unsettled_base = user_account
                    .unsettled_base
                    .checked_add(remaining)
                    .ok_or(ErrorCode::NumericalOverflow)?;
            }
        }
    }

    let order_book = &mut context.accounts.order_book;
    let removed = remove_order(order_book, order.order_id);
    require!(removed, ErrorCode::OrderNotFound);

    let user_account = &mut context.accounts.user_account;
    remove_open_order(user_account, order.order_id);

    order.status = OrderStatus::Cancelled;

    Ok(())
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [ORDER_BOOK_SEED, market.key().as_ref()],
        bump = order_book.bump
    )]
    pub order_book: Account<'info, OrderBook>,

    #[account(
        mut,
        seeds = [ORDER_SEED, market.key().as_ref(), order.order_id.to_le_bytes().as_ref()],
        bump = order.bump
    )]
    pub order: Account<'info, Order>,

    #[account(
        mut,
        seeds = [USER_ACCOUNT_SEED, market.key().as_ref(), owner.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub owner: Signer<'info>,
}
