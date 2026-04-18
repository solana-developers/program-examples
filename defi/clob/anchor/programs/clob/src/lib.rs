use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("C69UJ8irfmHq5ysyLek7FKApHR86FBeupiz4JnoyPzzx");

#[program]
pub mod clob {
    use super::*;

    /// Create a new market for a (base, quote) pair. Deploys the market PDA,
    /// the order book PDA, and the two PDA-authority vaults that hold locked
    /// funds while orders are open.
    pub fn initialize_market(
        context: Context<InitializeMarket>,
        fee_basis_points: u16,
        tick_size: u64,
        min_order_size: u64,
    ) -> Result<()> {
        instructions::initialize_market::handle_initialize_market(
            context,
            fee_basis_points,
            tick_size,
            min_order_size,
        )
    }

    /// Create a per-user, per-market account that tracks a user's open orders
    /// and unsettled balances.
    pub fn create_user_account(context: Context<CreateUserAccount>) -> Result<()> {
        instructions::create_user_account::handle_create_user_account(context)
    }

    /// Place a bid or ask. Locks the required funds (quote for bids, base for
    /// asks) into the market vault and inserts the order into the book at the
    /// correct price-time-priority position.
    pub fn place_order(
        context: Context<PlaceOrder>,
        side: state::OrderSide,
        price: u64,
        quantity: u64,
    ) -> Result<()> {
        instructions::place_order::handle_place_order(context, side, price, quantity)
    }

    /// Cancel an open (or partially filled) order. Credits the remaining
    /// locked amount back to the owner's unsettled balance; the actual token
    /// transfer happens on settle_funds.
    pub fn cancel_order(context: Context<CancelOrder>) -> Result<()> {
        instructions::cancel_order::handle_cancel_order(context)
    }

    /// Move accumulated unsettled balances out of the market vault and into
    /// the user's token accounts. No-op if both balances are zero.
    pub fn settle_funds(context: Context<SettleFunds>) -> Result<()> {
        instructions::settle_funds::handle_settle_funds(context)
    }
}
