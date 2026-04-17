use anchor_lang::prelude::*;

declare_id!("C69UJ8irfmHq5ysyLek7FKApHR86FBeupiz4JnoyPzzx");

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod clob {
    use super::*;

    pub fn initialize_market(
        context: Context<InitializeMarketAccountConstraints>,
        fee_basis_points: u16,
        tick_size: u64,
        min_order_size: u64,
    ) -> Result<()> {
        instructions::initialize_market(context, fee_basis_points, tick_size, min_order_size)
    }

    pub fn create_user_account(
        context: Context<CreateUserAccountAccountConstraints>,
    ) -> Result<()> {
        instructions::create_user_account(context)
    }

    pub fn place_order(
        context: Context<PlaceOrderAccountConstraints>,
        side: state::OrderSide,
        price: u64,
        quantity: u64,
    ) -> Result<()> {
        instructions::place_order(context, side, price, quantity)
    }

    pub fn cancel_order(context: Context<CancelOrderAccountConstraints>) -> Result<()> {
        instructions::cancel_order(context)
    }

    pub fn settle_funds(context: Context<SettleFundsAccountConstraints>) -> Result<()> {
        instructions::settle_funds(context)
    }
}
