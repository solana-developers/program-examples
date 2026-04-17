use anchor_lang::prelude::*;

pub const USER_ACCOUNT_SEED: &[u8] = b"user";

// Per-user, per-market account. Tracks open order ids and amounts owed back
// to the user (unsettled_*). Settlement moves those amounts from the vaults
// to the user's token accounts in settle_funds.
#[derive(InitSpace)]
#[account]
pub struct UserAccount {
    pub market: Pubkey,

    pub owner: Pubkey,

    pub unsettled_base: u64,

    pub unsettled_quote: u64,

    // 20 is chosen to match the matching engine's upper bound: a single user
    // shouldn't be able to spam the book. Keep the cap in sync with the
    // TooManyOpenOrders check in place_order.
    #[max_len(20)]
    pub open_orders: Vec<u64>,

    pub bump: u8,
}

pub fn add_open_order(account: &mut UserAccount, order_id: u64) {
    if !account.open_orders.contains(&order_id) {
        account.open_orders.push(order_id);
    }
}

pub fn remove_open_order(account: &mut UserAccount, order_id: u64) {
    if let Some(position) = account.open_orders.iter().position(|&id| id == order_id) {
        account.open_orders.remove(position);
    }
}
