use anchor_lang::prelude::*;

pub const ORDER_BOOK_SEED: &[u8] = b"order_book";

// Per-side capacity of the order book. 100 bids + 100 asks is a pragmatic
// ceiling for a teaching example: the whole OrderBook account stays under
// ~10 KB so it fits in a single transaction's account limit without needing
// realloc. Production CLOBs (Openbook, Phoenix) use zero-copy slabs to
// support tens of thousands of orders; that's out of scope here.
pub const MAX_ORDERS_PER_SIDE: usize = 100;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct OrderEntry {
    pub order_id: u64,

    pub price: u64,

    pub owner: Pubkey,
}

#[derive(InitSpace)]
#[account]
pub struct OrderBook {
    pub market: Pubkey,

    // Bids are sorted descending by price (best bid first).
    #[max_len(100)]
    pub bids: Vec<OrderEntry>,

    // Asks are sorted ascending by price (best ask first).
    #[max_len(100)]
    pub asks: Vec<OrderEntry>,

    pub next_order_id: u64,

    pub bump: u8,
}

pub fn add_bid(book: &mut OrderBook, order_id: u64, price: u64, owner: Pubkey) {
    let entry = OrderEntry { order_id, price, owner };
    let insert_position = book
        .bids
        .iter()
        .position(|bid| bid.price < price)
        .unwrap_or(book.bids.len());
    book.bids.insert(insert_position, entry);
}

pub fn add_ask(book: &mut OrderBook, order_id: u64, price: u64, owner: Pubkey) {
    let entry = OrderEntry { order_id, price, owner };
    let insert_position = book
        .asks
        .iter()
        .position(|ask| ask.price > price)
        .unwrap_or(book.asks.len());
    book.asks.insert(insert_position, entry);
}

pub fn remove_order(book: &mut OrderBook, order_id: u64) -> bool {
    if let Some(position) = book.bids.iter().position(|entry| entry.order_id == order_id) {
        book.bids.remove(position);
        return true;
    }

    if let Some(position) = book.asks.iter().position(|entry| entry.order_id == order_id) {
        book.asks.remove(position);
        return true;
    }

    false
}
