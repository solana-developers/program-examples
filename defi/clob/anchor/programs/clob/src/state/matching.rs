//! Matching engine helpers. Pure logic (no CPIs) that decides which resting
//! orders an incoming taker order would cross. The caller (place_order)
//! turns these decisions into token movements and account mutations.
//!
//! Price-time priority is implicit in the OrderBook's sorted Vecs:
//! - asks are sorted ascending (best ask first)
//! - bids are sorted descending (best bid first)
//! so "walk from index 0" is the correct price priority; and within a price
//! level, insertion order gives time priority (first-in fills first).

use crate::state::{OrderBook, OrderEntry, OrderSide};

/// One matched fill between the incoming taker order and a resting maker
/// order. `taker_remaining_after` is the taker's quantity after this fill
/// has been applied; matching stops when it reaches 0.
pub struct Fill {
    /// Index into `order_book.asks` (taker Bid) or `order_book.bids`
    /// (taker Ask) that this fill applies to. Kept so place_order can look
    /// the same entry up again to update its `quantity` on the book.
    pub resting_index: usize,

    /// order_id of the resting order being hit. place_order uses this to
    /// sanity-check the maker Order account passed as a remaining account.
    pub resting_order_id: u64,

    /// Quantity filled (in base tokens).
    pub fill_quantity: u64,

    /// Price at which the fill occurs (always the resting order's price —
    /// standard CLOB: maker's posted price wins; taker may get price
    /// improvement vs their limit).
    pub fill_price: u64,
}

/// Walk the opposite side of the book and produce the list of fills that
/// should occur for the incoming taker order. Does not mutate the book;
/// place_order applies the results. `resting_quantities` is indexed in
/// parallel with the resting side's Vec and gives each entry's current
/// quantity-remaining (which place_order tracks externally because the
/// book entry itself is just {order_id, price, owner} today — we pass it
/// in rather than storing it on OrderEntry so the existing on-chain layout
/// doesn't change).
///
/// Returns (fills, taker_remaining). taker_remaining is what's left over
/// that should rest on the book at the taker's limit price.
pub fn plan_fills(
    order_book: &OrderBook,
    resting_quantities: &[u64],
    incoming_side: OrderSide,
    incoming_price: u64,
    incoming_quantity: u64,
) -> (Vec<Fill>, u64) {
    let resting_entries: &Vec<OrderEntry> = match incoming_side {
        OrderSide::Bid => &order_book.asks,
        OrderSide::Ask => &order_book.bids,
    };

    let mut fills: Vec<Fill> = Vec::new();
    let mut taker_remaining = incoming_quantity;

    for (index, resting) in resting_entries.iter().enumerate() {
        if taker_remaining == 0 {
            break;
        }

        // Crossing condition: bid matches if incoming >= resting; ask
        // matches if incoming <= resting.
        let crosses = match incoming_side {
            OrderSide::Bid => incoming_price >= resting.price,
            OrderSide::Ask => incoming_price <= resting.price,
        };
        if !crosses {
            // Sides are sorted by price-priority, so once we fail to cross
            // we'll fail to cross every subsequent entry too.
            break;
        }

        let resting_remaining = resting_quantities[index];
        if resting_remaining == 0 {
            // Defensive: shouldn't happen because fully-filled orders are
            // removed from the book, but skip rather than crash.
            continue;
        }

        let fill_quantity = taker_remaining.min(resting_remaining);
        fills.push(Fill {
            resting_index: index,
            resting_order_id: resting.order_id,
            fill_quantity,
            fill_price: resting.price,
        });
        taker_remaining = taker_remaining.saturating_sub(fill_quantity);
    }

    (fills, taker_remaining)
}
