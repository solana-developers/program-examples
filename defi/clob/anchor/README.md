# Anchor CLOB

A minimal **Central Limit Order Book** (CLOB) on Solana. Users place limit buy (bid) or sell (ask) orders at a chosen price. Incoming orders cross against resting orders on the opposite side of the book using **price-time priority** — taker proceeds land in the user's `unsettled_*` balance and are withdrawn later via `settle_funds`. Unmatched remainders rest on the book as new maker orders.

This is a teaching example. It is deliberately small — the real CLOBs on Solana (Openbook, Phoenix) use zero-copy slab data structures and much more sophisticated matching, cancellation, and fee logic.

## Concepts

- **Market** — one trading pair, e.g. `BASE/QUOTE`. Stored at a PDA seeded by the two mints. The market account is the signer of its three token vaults (base, quote, fee).
- **Order Book** — a PDA per market holding two `Vec<OrderEntry>`s: bids (sorted descending by price) and asks (sorted ascending). Price-time priority is implicit in the Vec order: best price is index 0, and within a price level the earliest insertion is first.
- **User Account** — one per user per market. Tracks the user's open order ids and two "unsettled" balances (base and quote) representing tokens the program owes the user but has not yet transferred back.
- **Order** — a PDA per placed order, seeded by `(market, order_id)`. Stores price, original and filled quantity, status (`Open`, `PartiallyFilled`, `Filled`, `Cancelled`) and the owner.
- **Fee vault** — a separate token account (quote mint) that accumulates taker fees. The market PDA is its authority; only `withdraw_fees` can drain it, and only the market's stored `authority` may call that.

## Instructions

| Name                  | What it does |
|-----------------------|--------------|
| `initialize_market`   | Create the market, order book, base vault, quote vault, and fee vault for a `base/quote` pair. Sets fee (bps), tick size and minimum order size. |
| `create_user_account` | Initialise the caller's per-market user account. |
| `place_order`         | Lock the required funds (bids lock `price × quantity` of quote; asks lock `quantity` of base), then cross against the opposing side of the book (price-time priority). Taker proceeds land in `unsettled_base`/`unsettled_quote`; any unmatched remainder rests on the book. Callers pass resting-order PDAs and their owners' `UserAccount` PDAs as `remaining_accounts`, in pairs, in book order. |
| `cancel_order`        | Close an open (or partially filled) order. Credits the still-locked amount to the owner's `unsettled_base` / `unsettled_quote`. |
| `settle_funds`        | Move all unsettled base and quote from the market's vaults back to the owner's token accounts. Signs with the market PDA. |
| `withdraw_fees`       | Authority-only. Drains the fee vault into the authority's quote token account. Safe to call with an empty fee vault — it no-ops rather than reverting. |

### Matching semantics

`place_order` walks the opposite side of the book in price-time priority order:

- A **taker bid** walks asks lowest-first. For each ask whose price `<=` the bid's limit, a fill occurs at the ask's (maker's) price, for `min(taker_remaining, maker_remaining)` quantity. Stops when the bid is filled or the next ask's price exceeds the bid's limit.
- A **taker ask** mirrors: walk bids highest-first, fill at the bid's price while the bid's price `>=` the ask's limit.
- **Price improvement** — a bid at 1000 crossing an ask at 900 fills at 900. The taker locked `1000 × qty` of quote up front; the `100 × qty` they didn't need is refunded to their `unsettled_quote`.
- **Time priority** — two orders at the same price fill in the order they were inserted. The oldest resting order wins.

### Fee model

A single `fee_basis_points` value (0–10_000) applies to the taker fee on the quote side of each fill:

```
gross  = fill_price * fill_quantity
fee    = gross * fee_basis_points / 10_000   # rounded down
```

- The fee is deducted from the gross quote flowing between the two traders, and transferred to the market's `fee_vault` via one CPI per `place_order` call (aggregated across fills to keep CU cost down).
- In this example the fee is effectively maker-funded (the maker receives `gross − fee`) rather than taker-funded (where the taker would bring extra quote to cover the fee on top of the gross). This keeps the instruction simple — no per-fill CPI from the taker's ATA — and matches how Openbook v2 and Phoenix tend to operate. If you need strictly maker-neutral fees, add a second `transfer_checked` from the taker's ATA to the `fee_vault` for each fill.
- Makers never pay an explicit maker fee in this example.

### Remaining accounts

`place_order`'s matching needs to mutate each resting maker's `Order` (to bump `filled_quantity` and flip `status`) and their `UserAccount` (to credit `unsettled_*` and drop filled orders from `open_orders`). Those accounts are passed as `remaining_accounts` in pairs:

```
remaining_accounts = [
    maker_1_order, maker_1_user_account,
    maker_2_order, maker_2_user_account,
    ...
]
```

Ordered the way the book will walk them: lowest-priced ask first for a taker bid, highest-priced bid first for a taker ask. The program re-verifies the pairs against the live order book (rejecting out-of-order or unknown order ids) before applying any fills.

## Build

```shell
anchor build
```

## Test

```shell
anchor test
```

Tests are pure Rust, running against [LiteSVM](https://github.com/LiteSVM/litesvm). They live in `programs/clob/tests/test_clob.rs` and include the built `.so` via `include_bytes!`, so a fresh `anchor build` must run first. `anchor test` does this automatically; alternatively run `anchor build && cargo test`.

## Credit

Ported and modernised from [anchor-decentralized-exchange-clob](https://github.com/mikemaccana/anchor-decentralized-exchange-clob). Migrated from Anchor 0.32.1 to Anchor 1.0.0 and conformed to the repo's LiteSVM-Rust-tests convention (no magic numbers, `Box`-ed interface accounts to keep BPF stack size within budget). Matching engine added in a subsequent pass.
