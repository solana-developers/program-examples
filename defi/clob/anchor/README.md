# Anchor CLOB

A minimal **Central Limit Order Book** (CLOB) on Solana. Users place limit buy (bid) or sell (ask) orders at a chosen price; their tokens sit in a program-owned vault until the order is cancelled. Cancellation credits the refund to an internal balance and a later `settle_funds` call moves those tokens back to the user.

This is a teaching example. It is deliberately small — the real CLOBs on Solana (Openbook, Phoenix) use zero-copy slab data structures and much more sophisticated matching and fee logic.

## Concepts

- **Market** — one trading pair, e.g. `BASE/QUOTE`. Stored at a PDA seeded by the two mints. The market account is the signer of its two token vaults.
- **Order Book** — a PDA per market holding two `Vec<OrderEntry>`s: bids (sorted descending by price) and asks (sorted ascending). Price-time priority is implicit in the order they are inserted.
- **User Account** — one per user per market. Tracks the user's open order ids and two "unsettled" balances (base and quote) representing tokens the program owes the user but has not yet transferred back.
- **Order** — a PDA per placed order, seeded by `(market, order_id)`. Stores price, original and filled quantity, status (`Open`, `PartiallyFilled`, `Filled`, `Cancelled`) and the owner.

## Instructions

| Name                  | What it does |
|-----------------------|--------------|
| `initialize_market`   | Create the market, order book and two token vaults for a `base/quote` pair. Sets fee (bps), tick size and minimum order size. |
| `create_user_account` | Initialise the caller's per-market user account. |
| `place_order`         | Add a limit order to the book and lock the funds it would need if filled: bids lock `price × quantity` of quote; asks lock `quantity` of base. |
| `cancel_order`        | Close an open (or partially filled) order. Credits the still-locked amount to the owner's `unsettled_base` / `unsettled_quote`. |
| `settle_funds`        | Move all unsettled base and quote from the market's vaults back to the owner's token accounts. Signs with the market PDA. |

### Scope note

The program stores the book and locks funds on placement, but does **not** currently run a matching engine inside `place_order`. Crossed orders (a bid at or above the best ask) will sit side-by-side in the book rather than trade. Adding matching requires passing the opposing orders (and their owners' user accounts and token accounts) as remaining accounts and clearing the filled amounts across both sides; it's a natural next extension.

## Build

```shell
anchor build
```

## Test

```shell
anchor test --validator legacy
```

The `--validator legacy` flag is required: Anchor 1.0's default "surfpool" validator does not yet accept the websocket RPC methods Solana Kit uses for transaction confirmation, so tests hang waiting for their first transaction. `solana-test-validator` works.

The test script (defined in `Anchor.toml`) first runs `npx create-codama-clients` to generate a TypeScript client from the built IDL into `dist/clob-client/`, then executes the `node:test` suite with `tsx`.

## Credit

Ported and modernised from [anchor-decentralized-exchange-clob](https://github.com/mikemaccana/anchor-decentralized-exchange-clob). Migrated from Anchor 0.32.1 to Anchor 1.0.0 and conformed to the [Solana Anchor coding skill](https://github.com/mikemaccana/solana-anchor-claude-skill) (Kit + Kite + Codama, `node:test`, no `@coral-xyz/anchor`, no magic numbers, `Box`-ed interface accounts to keep BPF stack size within budget).
