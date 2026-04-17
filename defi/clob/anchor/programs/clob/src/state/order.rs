use anchor_lang::prelude::*;

pub const ORDER_SEED: &[u8] = b"order";

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
}

#[derive(InitSpace)]
#[account]
pub struct Order {
    pub market: Pubkey,

    pub owner: Pubkey,

    pub order_id: u64,

    pub side: OrderSide,

    pub price: u64,

    pub original_quantity: u64,

    pub filled_quantity: u64,

    pub status: OrderStatus,

    pub timestamp: i64,

    pub bump: u8,
}

pub fn remaining_quantity(order: &Order) -> u64 {
    order.original_quantity.saturating_sub(order.filled_quantity)
}
