# CLOB — Central Limit Order Book

An Anchor program that runs a simple **onchain order book** for a single
SPL-token trading pair. Users post buy or sell offers at the prices they
want, the program matches crossing offers, and settles the resulting
token movements.

This README is a teaching document. If you have never written a Solana
program before and have no background in trading, you are the target
reader. Every term that could be unfamiliar is explained the first time
it appears, and every instruction is walked through step by step with
the exact token movements it causes.

If you already know what an order book, a limit order and a taker fee
are, skip to [Accounts and PDAs](#3-accounts-and-pdas) or
[Instruction lifecycle walkthrough](#4-instruction-lifecycle-walkthrough).

---

## Table of contents

1. [What does this program do?](#1-what-does-this-program-do)
2. [Glossary](#2-glossary)
3. [Accounts and PDAs](#3-accounts-and-pdas)
4. [Instruction lifecycle walkthrough](#4-instruction-lifecycle-walkthrough)
5. [The matching engine — step by step](#5-the-matching-engine--step-by-step)
6. [Full-lifecycle worked examples](#6-full-lifecycle-worked-examples)
7. [Safety and edge cases](#7-safety-and-edge-cases)
8. [Running the tests](#8-running-the-tests)
9. [Extending the program](#9-extending-the-program)

---

## 1. What does this program do?

Two users want to swap tokens at prices they each picked:

- Alice holds some amount of SPL mint **Q** (the "quote" mint — think of
  this as the pricing currency, like USD in "BTC is $60 000") and wants
  to obtain some amount of SPL mint **B** (the "base" mint — the asset
  being priced), but only if she can get B at 900 Q per unit or lower.
- Bob holds mint **B** and wants Q, but only if he can get at least 950
  Q per unit of B he sells.

They post their offers — Alice a **bid** (buy offer) at price 900, Bob
an **ask** (sell offer) at price 950 — and wait. Alice's bid sits on
the book. Bob's ask sits on the book. Neither crosses the other, so
nothing happens yet.

Later, Carol shows up holding B and willing to sell at any price ≥ 900.
She posts an ask at 900. Now Alice's bid (900) crosses Carol's new ask
(900). The program:

1. Pairs them up.
2. Takes Carol's B out of Carol's token account, locks it in the
   program's vault.
3. Takes Alice's Q out of Alice's token account (it was already locked
   there when Alice placed her bid).
4. Credits each of them with what they're owed, minus a fee for the
   market operator.

At no point does either of them transfer directly to the other — all
token flows go through two program-owned vaults, and both users later
call `settle_funds` to pull their balances out.

### The onchain pieces, in plain terms

- A **Market** PDA — one per base/quote pair. Stores fee rate, tick
  size, minimum order size, the addresses of the four related accounts
  (base vault, quote vault, fee vault, order book), and the pubkey
  that can withdraw accumulated fees.
- An **OrderBook** PDA — two sorted lists (bids best-first, asks
  best-first) of lightweight `OrderEntry` records, each pointing at a
  full `Order` account.
- A **UserAccount** PDA — one per `(market, wallet)` pair. Tracks the
  order_ids this user has open and two running tallies
  (`unsettled_base`, `unsettled_quote`) of tokens owed back to this
  user from fills or cancellations.
- An **Order** PDA — one per placed order. Stores price, quantity,
  side (bid or ask), fill status, and the owner.
- Three token accounts held by the Market PDA: `base_vault` (all
  sellers' locked base + buyers' bought base waiting to be withdrawn),
  `quote_vault` (mirror for quote), and `fee_vault` (accumulated taker
  fees).

### Tradfi background, briefly

For readers new to trading terms — two quick sentences per concept.
They're optional; everything above already describes the program
mechanically.

- **A limit order** is an instruction to trade an amount of asset at a
  specific price or better. A *bid* is a limit order to buy, an *ask*
  is a limit order to sell. The "limit" part means: don't trade at a
  worse price than the one I named.

- **An order book** is just the currently-open bids and asks, usually
  sorted so the best price on each side sits at the top. The "top of
  book" on the bid side is the highest-priced buy offer; the top of
  book on the ask side is the lowest-priced sell offer.

- **A maker** is whoever posts an order that doesn't immediately match
  — they "make" liquidity by leaving their offer on the book for
  others to trade against. A **taker** is whoever walks into the book
  and hits the resting orders — they "take" liquidity.

- **A taker fee** is a small cut of each trade that the market
  operator takes from the taker's leg of the trade. Expressed in basis
  points (see glossary), so 50 bps = 0.5%.

- **Price-time priority** is the universal ordering rule: at the same
  price level, whoever posted first fills first.

- **Settlement** is the step that actually moves tokens out of the
  custody account and back to the user. This program splits matching
  and settlement into two instructions (`place_order` + `settle_funds`)
  so a taker crossing a long list of orders doesn't have to pay for a
  token CPI per maker.

### What this example is not

- **Not deployed, not audited.** Treat as a learning example. The
  OrderBook is a `Vec<OrderEntry>` with a 100-per-side cap that
  deserialises in full every call — fine at small scale, unsuitable
  for production. Real Solana CLOBs (Openbook v2, Phoenix) use
  zero-copy slabs.
- **No explicit IOC / FOK / post-only** — every order matches what it
  can and rests the rest.
- **No circuit breakers, no oracles, no price bands.**

---

## 2. Glossary

Terms used below, defined in terms of what they are mechanically.

**Account**
: On Solana, every piece of state lives in an *account* — a 32-byte
address, some lamports keeping it rent-exempt, an owner program, and
a byte buffer. Wallets, token balances, and program config are all
accounts.

**Lamport**
: The smallest unit of SOL. 1 SOL = 10⁹ lamports.

**Signer**
: An account whose private key signed the transaction. A signer is the
only thing that can authorise transfers out of an account it owns.

**SPL token**
: Solana's ERC-20 equivalent. An SPL *mint* describes a token; each
user's balance lives in a separate *token account* owned by the SPL
Token program.

**Token account**
: An account holding a balance of one mint, with an *authority* pubkey
that can move tokens out. Authorities are usually user wallets but can
be PDAs — in this program the Market PDA is the authority of all
three vaults.

**ATA (Associated Token Account)**
: The conventional, deterministic token account for a `(wallet, mint)`
pair. "Sending USDC to someone's wallet" really means sending to their
USDC ATA.

**PDA (Program Derived Address)**
: A deterministic address derived from a list of byte "seeds" plus a
program id. PDAs have no private key. A program *signs* as a PDA by
re-supplying the seeds during a CPI. This program creates four kinds
of PDA: `Market`, `OrderBook`, `Order`, `UserAccount`. The vaults are
regular token accounts (not PDAs) whose authority is set to the
`Market` PDA.

**Seeds**
: Bytes that, with the program id, derive a PDA. This program's seeds:

    Market        ["market", base_mint, quote_mint]
    OrderBook     ["order_book", market]
    Order         ["order", market, order_id.to_le_bytes()]
    UserAccount   ["user",   market, owner]

**Bump**
: The byte offset that makes `find_program_address` produce an
off-curve address. Stored on each PDA struct so the program doesn't
recompute it every time it signs.

**CPI (Cross-Program Invocation)**
: One program calling another inside the same transaction. This
program's CPIs go to the SPL Token program's `TransferChecked`.

**Discriminator**
: First 8 bytes of each Anchor account — the first 8 bytes of
`sha256("account:<StructName>")`. Anchor writes it at initialisation
and rejects any deserialisation where the prefix doesn't match.

**Basis point (bps)**
: 1/100 of a percent. 10 000 bps = 100%. The program's fee rate is
expressed in bps (`fee_basis_points: u16`).

**Base asset, quote asset**
: Two words for the two sides of a pair. In "BASE/QUOTE", the base is
the asset being priced and the quote is the pricing unit. Bids spend
quote and receive base; asks spend base and receive quote.

**Bid, ask**
: A bid is a buy order (sits on the bid side of the book). An ask is a
sell order. On this program they're the two variants of `OrderSide`.

**Limit price**
: The worst price at which an order is allowed to trade — for a bid,
the *highest* the taker is willing to pay; for an ask, the *lowest*
the seller is willing to accept. A bid at 900 will not fill against
an ask at 950; it rests at 900 until a seller drops their price.

**Tick size**
: Smallest allowable price increment. A market with `tick_size = 10`
accepts prices 10, 20, 30, … but rejects 15. Stops the book filling
up with 1-unit-apart orders.

**Minimum order size**
: Smallest allowable `quantity` for any order. Keeps dust orders from
polluting the book.

**Maker, taker**
: A maker is whoever posted the resting order that gets hit. A taker
is whoever walked in and hit it. The same person is sometimes both in
one call to `place_order`: they fill some quantity as a taker and
rest the rest as a maker for the next person.

**Match, fill, cross**
: Two orders *cross* when the bid's price is ≥ the ask's price. They
*match* (are paired up) and a *fill* is the result — one crossing
event, with a fill quantity and a fill price. A single `place_order`
call can produce many fills if the taker quantity eats through
several resting orders.

**Price improvement**
: When a taker's limit is better than the best resting price on the
other side, the fill happens at the resting price (maker's price
wins). The taker got a better deal than they asked for — the
difference is *price improvement*. In this program that's reflected
by refunding the difference to the taker's `unsettled_quote`.

**Unsettled balance**
: Two `u64` counters on each `UserAccount`: `unsettled_base` and
`unsettled_quote`. Fills, price-improvement rebates, and order
cancellations all increase these counters. The physical tokens still
sit in the market's vaults. `settle_funds` moves them to the user's
own token accounts and zeroes the counters.

**Fee vault**
: A separate SPL token account (quote mint) owned by the Market PDA.
Every taker fee — `gross * fee_bps / 10_000` per fill — moves here in
one batched CPI at the end of `place_order`.

**Price-time priority**
: Best price first; at the same price, earliest-posted first. Here
price priority is enforced by keeping `bids` sorted descending and
`asks` sorted ascending, and time priority falls out of insertion
order at each level (new orders at an existing price go to the end of
that price's run).

**Remaining accounts**
: Solana lets the caller pass a tail of extra `AccountInfo`s beyond
the ones named in `#[derive(Accounts)]`. This program uses them for
maker orders: for each resting order the taker wants to cross, the
caller supplies `(maker_order_pda, maker_user_account_pda)` in the
book's price-time order. The handler walks them in pairs.

---

## 3. Accounts and PDAs

### State / data accounts

| Account | PDA? | Seeds | Authority | Holds |
|---|---|---|---|---|
| `Market` | yes | `["market", base_mint, quote_mint]` | program | fee rate, tick size, min order size, base/quote mint pubkeys, vault pubkeys, order book pubkey, `authority` wallet (allowed to withdraw fees) |
| `OrderBook` | yes | `["order_book", market]` | program | two `Vec<OrderEntry>` (bids best-first, asks best-first), `next_order_id` |
| `Order` | yes | `["order", market, order_id.to_le_bytes()]` | program | owner, side, price, original_quantity, filled_quantity, status, timestamp |
| `UserAccount` | yes | `["user", market, owner]` | program | `unsettled_base`, `unsettled_quote`, `open_orders: Vec<u64>` (max 20) |

### Token accounts (owned by SPL Token, authority = Market PDA)

| Account | PDA? | Authority | Mint | Holds |
|---|---|---|---|---|
| `base_vault` | no (regular token account) | Market PDA | base | bids' locked base IS NOT STORED HERE — only asks' locked base sits here pre-match, plus base owed to bid-takers waiting for `settle_funds` |
| `quote_vault` | no | Market PDA | quote | bids' locked quote pre-match, plus quote owed to ask-takers and bid-makers waiting for settlement |
| `fee_vault` | no | Market PDA | quote | taker fees accumulated across all fills; drained by `withdraw_fees` |

Note: the **token vaults are not PDAs**. They are regular token
accounts created with `init` in `initialize_market.rs`; their
*authority* is the Market PDA, so only the program can move funds out.
Their addresses are computed by the caller (e.g. generated Keypairs in
the tests) and then written to `market.base_vault` / `quote_vault` /
`fee_vault` for the program to validate them on later calls via
`has_one = fee_vault` etc.

### `OrderEntry` layout on `OrderBook`

```rust
pub struct OrderEntry {
    pub order_id: u64,  // links to the full Order PDA
    pub price: u64,
    pub owner: Pubkey,
}
```

Kept deliberately small so the OrderBook account stays under the 10 KB
limit with 100 bids + 100 asks. The full order state (quantity,
filled_quantity, status, timestamp) lives on the `Order` PDA; the book
just needs enough to pick what to cross next and re-derive the PDA.

### `Order` state

From [`state/order.rs`](programs/clob/src/state/order.rs):

```rust
pub struct Order {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub order_id: u64,
    pub side: OrderSide,           // Bid | Ask
    pub price: u64,
    pub original_quantity: u64,
    pub filled_quantity: u64,
    pub status: OrderStatus,       // Open | PartiallyFilled | Filled | Cancelled
    pub timestamp: i64,
    pub bump: u8,
}
```

`remaining_quantity(order) = original_quantity - filled_quantity`. Used
by `cancel_order` to decide how much to credit back to the user.

### `UserAccount` state

```rust
pub struct UserAccount {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub unsettled_base: u64,
    pub unsettled_quote: u64,
    pub open_orders: Vec<u64>,   // capped at 20 via Anchor max_len
    pub bump: u8,
}
```

The `open_orders` cap (20 per user) is mirrored by a
`MAX_OPEN_ORDERS_PER_USER` check in `place_order`. One user cannot
flood the book.

### How vault balances evolve

At any point in time:

- `base_vault.balance` = sum of all resting asks' `remaining_quantity`
  + every user's `unsettled_base`.
- `quote_vault.balance` = sum of all resting bids'
  `price * remaining_quantity`
  + every user's `unsettled_quote`.

(Plus the bit of quote that the matching engine has already taken out
as fee and batched into `fee_vault`.)

This is not a hard invariant the program enforces — it emerges from
the flows. The invariant worth caring about is the per-event balance:
every fill moves tokens from the loser's locked pool to the winner's
`unsettled_*`, plus the fee cut to `fee_vault`. The unit tests check
this directly (`settle_funds_after_match_pays_out_both_unsettled_balances`).

---

## 4. Instruction lifecycle walkthrough

The program has six instructions. The order a user encounters them is:

1. `initialize_market` (market operator — once)
2. `create_user_account` (every user, once per market)
3. `place_order` (a user — as many times as they want)
4. `cancel_order` (a user — to remove a resting order)
5. `settle_funds` (a user — to collect winnings)
6. `withdraw_fees` (market authority — to collect protocol revenue)

For each, the shape is: who signs, what accounts go in, what PDAs get
created, what token flows happen, what state mutates, what checks are
run.

Token flow shorthand:

```
  <source> --[amount of <mint>]--> <destination>
```

### 4.1 `initialize_market`

**Who calls it:** the market operator. They create a new trading pair.

**Signers:** `authority`.

**Parameters:**

```rust
pub fn initialize_market(
    context: Context<InitializeMarket>,
    fee_basis_points: u16,
    tick_size: u64,
    min_order_size: u64,
) -> Result<()>
```

**Accounts in:**

- `authority` (signer, mut — pays account rent for all five new
  accounts)
- `market` (PDA, **init**, seeds `["market", base_mint, quote_mint]`)
- `order_book` (PDA, **init**, seeds `["order_book", market]`)
- `base_mint`, `quote_mint` (read-only)
- `base_vault`, `quote_vault`, `fee_vault` (all **init** as
  `TokenAccount`s, authority = `market`)
- `token_program`, `system_program`

**Checks:**

- `tick_size > 0` → `InvalidTickSize`
- `min_order_size > 0` → `BelowMinOrderSize`
- `fee_basis_points <= 10_000` → `InvalidFeeBasisPoints`

**Token movements:** none (the vaults are empty after init).

**State changes:** `market` and `order_book` accounts are written with
the supplied parameters plus all the derived fields
(`market.authority`, the vault pubkeys, `is_active = true`,
`next_order_id = 1`).

The vaults are regular SPL token accounts, *not* PDAs — their
addresses are chosen by the caller (typically fresh keypairs) and
captured on the market's state so later instructions can validate
them.

### 4.2 `create_user_account`

**Who calls it:** every user, exactly once per market they want to
trade on.

**Signers:** `owner`.

**Accounts in:**

- `owner` (signer, mut — pays rent)
- `market` (read-only)
- `user_account` (PDA, **init**, seeds `["user", market, owner]`)
- `system_program`

**Token movements:** none.

**State changes:** new `UserAccount` with all counters zero and no
open orders.

### 4.3 `place_order`

**Who calls it:** anyone with a `UserAccount` for this market.

**Signers:** `owner`.

**Parameters:**

```rust
pub fn place_order<'info>(
    context: Context<'info, PlaceOrder<'info>>,
    side: OrderSide,   // Bid | Ask
    price: u64,
    quantity: u64,
) -> Result<()>
```

**Accounts in (named):**

- `market` (mut, `has_one = fee_vault`)
- `order_book` (mut, PDA seeds-checked)
- `order` (PDA, **init**, seeds
  `["order", market, next_order_id.to_le_bytes()]`)
- `user_account` (mut, PDA seeds-checked)
- `base_vault`, `quote_vault`, `fee_vault` (all mut, boxed)
- `user_base_account`, `user_quote_account` (mut — the caller's ATAs)
- `base_mint`, `quote_mint` (read-only)
- `owner` (signer, mut)
- `token_program`, `system_program`

**Accounts in (remaining):** a list of `AccountInfo`s passed via the
transaction's remaining accounts, grouped in pairs. For each resting
order the caller wants the taker to cross, in the book's price-time
order:

```
remaining_accounts[2*i]     = maker_order_pda (Order account)
remaining_accounts[2*i + 1] = maker_user_account_pda (UserAccount)
```

If the caller doesn't pass any pairs, the order is treated as
pure-maker: whatever part of it is allowed by the book state becomes a
resting order.

**Checks (top of handler):**

- `market.is_active` → `MarketPaused`
- `price > 0` → `InvalidPrice`
- `price % tick_size == 0` → `InvalidTickSize`
- `quantity >= min_order_size` → `BelowMinOrderSize`
- `open_orders.len() < 20` (mirror of the max_len on the struct) →
  `TooManyOpenOrders`
- `remaining_accounts.len() % 2 == 0` → `MissingMakerAccounts`

**Checks (per maker pair, during planning):**

- Maker order's `order_id` exists in the relevant book side →
  `MakerAccountMismatch`
- Maker order's `market == market.key()` → `MakerAccountMismatch`
- Maker pair index == book position (i.e. caller walked the book in
  order) → `MakerAccountMismatch`

**Checks (per fill, during execution):**

- Maker order and user account have matching `owner` →
  `MakerOwnerMismatch`
- Maker user account's `market == market.key()` →
  `MakerAccountMismatch`

**Checks (before resting remainder):**

- `bids.len() + asks.len() < 2 * MAX_ORDERS_PER_SIDE` →
  `OrderBookFull`
- Integer math throughout: every multiplication uses
  `checked_mul`; every addition on balances uses `checked_add`;
  fee division uses `u128` to avoid intermediate overflow →
  `NumericalOverflow`

**Token movements (up front):**

For a **bid**:
```
  user_quote_account --[price * quantity of quote_mint]--> quote_vault
```

For an **ask**:
```
  user_base_account --[quantity of base_mint]--> base_vault
```

The full lock happens regardless of whether the order will fully fill
immediately. That keeps the vault invariant simple: the token account
always holds *exactly* what's needed to fulfil every open position
plus every unsettled balance.

**Token movements (during matching, per fill):** see
[§5. The matching engine — step by step](#5-the-matching-engine--step-by-step).
Summary:

- For a taker bid crossing a resting ask at price `p`:
  ```
  quote_vault         --[p * fill_qty * fee_bps / 10_000]--> fee_vault
  (everything else stays in quote_vault as unsettled_quote for maker)
  (base_vault provides the taker's base via unsettled_base — the base
   was pre-locked when the maker placed their ask)
  ```

- For a taker ask crossing a resting bid at price `p`:
  ```
  quote_vault         --[p * fill_qty * fee_bps / 10_000]--> fee_vault
  ```

No user's ATA is touched during matching — all movements happen
between vaults or inside `UserAccount` counters. Physical payouts wait
for `settle_funds`.

**PDAs created:** `order` (always; even fully-crossed takers get an
Order PDA, marked `Filled` immediately, for consistency with
indexers).

**State changes:**

On the taker's `UserAccount`:

- `unsettled_base += sum of fill.fill_quantity` (taker bid side)
- `unsettled_quote += sum of price_improvement_rebate`
  (taker bid side, per fill)
- `unsettled_quote += sum of (gross - fee)` (taker ask side)

On each maker's `Order` (via `Account::try_from` + `exit`):

- `filled_quantity += fill.fill_quantity`
- `status = PartiallyFilled` or `Filled`

On each maker's `UserAccount`:

- `unsettled_quote += gross - fee` (maker was an ask)
- `unsettled_base += fill.fill_quantity` (maker was a bid)
- `open_orders` list: maker's order removed if fully filled

On `order_book`:

- `next_order_id += 1`
- Fully-filled makers removed from the relevant side (bids or asks) in
  reverse-index order
- Taker's remainder (if any) inserted into the correct side in price
  order

On the caller's new `order`:

- All fields populated
- `status = Filled` if taker fully matched; otherwise
  `PartiallyFilled` (if some fills) or `Open` (if no fills)

### 4.4 `cancel_order`

**Who calls it:** the order's owner.

**Signers:** `owner`.

**Accounts in:**

- `market`
- `order_book` (mut)
- `order` (mut, PDA seeds-checked via stored bump)
- `user_account` (mut)
- `owner` (signer)

**Checks:**

- `order.owner == owner.key()` → `Unauthorized`
- `order.status ∈ {Open, PartiallyFilled}` → `OrderNotCancellable`
- The order's `order_id` is present in `order_book` → `OrderNotFound`
  (sanity — shouldn't normally fire since fully-filled orders aren't
  cancellable)

**Token movements:** none. Cancellation is an accounting-only step.

**State changes:**

- For a cancelled bid: `unsettled_quote += price * remaining_quantity`
  (the quote the bid had locked in the vault is now owed back to the
  owner).
- For a cancelled ask: `unsettled_base += remaining_quantity`.
- Remove from `order_book.bids` or `order_book.asks`.
- Remove from `user_account.open_orders`.
- `order.status = Cancelled`.

The actual token move happens on the next `settle_funds` call.

### 4.5 `settle_funds`

**Who calls it:** any user. No-op when both unsettled counters are
zero, so it is safe to call on a heartbeat/cron.

**Signers:** `owner`.

**Accounts in:**

- `market` (mut)
- `user_account` (mut)
- `base_vault`, `quote_vault` (mut, boxed)
- `user_base_account`, `user_quote_account` (mut, boxed — caller's
  ATAs; caller must create them before calling)
- `base_mint`, `quote_mint` (boxed, read-only)
- `owner` (signer)
- `token_program`

**Checks:** none beyond Anchor's account-validation (ownership,
mint checks on token accounts, PDA seeds).

**Token movements:**

```
  base_vault  --[user_account.unsettled_base of base_mint]--> user_base_account
  quote_vault --[user_account.unsettled_quote of quote_mint]--> user_quote_account
```

Both transfers are CPIs to the SPL Token program, signed by the
`Market` PDA using seeds `["market", base_mint, quote_mint, bump]`.

**State changes:**

- `user_account.unsettled_base = 0`
- `user_account.unsettled_quote = 0`

### 4.6 `withdraw_fees`

**Who calls it:** the market authority (whichever pubkey was set as
`market.authority` at initialisation).

**Signers:** `authority`.

**Accounts in:**

- `market` (mut, `has_one = fee_vault`)
- `fee_vault` (mut, boxed)
- `authority_quote_account` (mut, boxed — destination)
- `quote_mint` (boxed)
- `authority` (signer)
- `token_program`

**Checks:**

- `authority.key() == market.authority` → `NotMarketAuthority`
- If `fee_vault.amount == 0`, returns `Ok(())` silently (so this call
  is cheap to schedule)

**Token movements:**

```
  fee_vault --[fee_vault.balance of quote_mint]--> authority_quote_account
```

Signed by the Market PDA.

**State changes:** none on program state (the vault balance drops to
zero as a side effect of the transfer).

---

## 5. The matching engine — step by step

This is the heart of the program. Everything in `place_order` after
the initial fund lock is matching-engine work. Follow along with
[`place_order.rs`](programs/clob/src/instructions/place_order.rs) and
[`state/matching.rs`](programs/clob/src/state/matching.rs) — it'll
read more easily once you've gone through this section.

### 5.1 The plan

1. Caller passes `(side, price, quantity)` and, in remaining_accounts,
   the maker pairs to cross against.
2. The handler locks the required funds into the vault (done up
   front, before any matching — see §4.3).
3. **Plan the fills** (pure logic, no mutations): walk the opposite
   side of the book in price order. For each entry whose price
   crosses the taker's limit, record a `Fill { resting_index,
   resting_order_id, fill_quantity, fill_price }`. Stop when either
   the taker's quantity is exhausted or the next entry fails to
   cross.
4. **Apply the fills** (mutate state): for each fill, update the
   maker's `Order` (increment `filled_quantity`, flip status), update
   the maker's `UserAccount` (credit `unsettled_base` or
   `unsettled_quote`), and accumulate deltas for the taker.
5. **Clean the book**: remove fully-filled makers from the relevant
   side of `order_book.bids`/`asks`, in reverse-index order.
6. **Pay the fee**: one batched CPI from `quote_vault` to `fee_vault`
   for the sum of per-fill fees.
7. **Apply the taker deltas**: single mutation of the taker's
   `UserAccount`.
8. **Rest the remainder**: if `taker_remaining > 0`, insert the
   new `Order` into the book at the taker's limit price, add its
   `order_id` to the taker's `open_orders`, set status to
   `PartiallyFilled` (if any fills) or `Open` (if none).

### 5.2 Why bids spend quote, asks spend base — the full accounting

Pick a taker **bid** at price `bp` and quantity `bq`, crossing a
resting **ask** at `ap ≤ bp` with remaining quantity `aq`. Let
`fill_qty = min(bq, aq)` and `fill_price = ap` (maker's price wins).

Per-fill quantities:

```
gross       = fill_price * fill_qty                         (quote tokens)
fee         = gross * fee_bps / 10_000                       (quote tokens)
net_to_maker = gross - fee                                   (quote tokens)
locked      = bp * fill_qty                                  (quote tokens the taker had locked for this fill)
rebate      = locked - gross                                 (quote the taker locked but doesn't need to spend)
```

Token flows:

```
  quote_vault  --[fee]---------> fee_vault       (CPI signed by Market PDA, batched across all fills)

  # No physical transfer for the base and net-quote legs — they stay in the
  # vaults, accounted for via unsettled_* counters:

  maker.unsettled_quote += net_to_maker          (maker collects gross - fee)
  taker.unsettled_base  += fill_qty              (taker gets the base)
  taker.unsettled_quote += rebate                (price improvement refund)
```

The *base* that the taker now owns was already in `base_vault` —
remember, the maker locked it there when placing the ask. The *quote*
that the maker now owns was already in `quote_vault` — the taker
locked `bp * bq` there at the top of this call. Nothing leaves the
vaults except the fee. Everything else gets paid out later, on
`settle_funds`.

For the opposite direction — a taker **ask** at `ap` crossing a
resting **bid** at `bp ≥ ap`:

```
fill_qty     = min(taker_remaining, bp_remaining)
fill_price   = bp
gross        = bp * fill_qty
fee          = gross * fee_bps / 10_000
net_to_taker = gross - fee

Token flows:
  quote_vault --[fee]------> fee_vault

  taker.unsettled_quote += net_to_taker
  maker.unsettled_base  += fill_qty
```

No rebate on this side: the maker's bid locked exactly `bp *
bid_original_qty` of quote up front, and of that, `bp * fill_qty` is
being spent right now at exactly that price — no leftover.

### 5.3 Worked example — taker bid crosses two resting asks

Start with an empty book. Fees 10 bps (0.1%). Tick size 1.

1. Maker Dan posts an ask at price 900, quantity 5. `place_order(Ask,
   900, 5)`. Dan's token account loses 5 base; base_vault gains 5
   base. `order_book.asks = [(id=1, price=900)]`.

2. Maker Erin posts an ask at price 950, quantity 5. Same mechanism.
   `base_vault.balance = 10`. `order_book.asks = [(1, 900), (2, 950)]`
   (ascending).

3. Taker Faye places a bid at 1000 for quantity 7. She passes both
   makers as remaining_accounts: `(order_1, dan_user), (order_2,
   erin_user)`.

   Step A — lock. Faye's quote ATA loses `1000 * 7 = 7000` quote;
   `quote_vault.balance += 7000`.

   Step B — plan:
   - Fill 0: resting index 0 (Dan's ask), order_id 1, qty = min(7,
     5) = 5, price = 900. `taker_remaining = 7 - 5 = 2`.
   - Fill 1: resting index 1 (Erin's ask), order_id 2, qty = min(2,
     5) = 2, price = 950. `taker_remaining = 0`.

   Step C — apply fills:

   For Fill 0 (Dan):
   - gross = 900 * 5 = 4500; fee = 4500 * 10 / 10 000 = 4;
     net_to_maker = 4496.
   - `dan_user_account.unsettled_quote += 4496`
   - `faye_user_account.unsettled_base += 5`
   - Faye's rebate = 1000*5 − 4500 = 500.
     `faye_user_account.unsettled_quote += 500`
   - `dan_order.filled_quantity = 5`, status = Filled,
     remove from `dan_user_account.open_orders`.

   For Fill 1 (Erin):
   - gross = 950 * 2 = 1900; fee = 1; net_to_maker = 1899.
   - `erin_user_account.unsettled_quote += 1899`
   - `faye_user_account.unsettled_base += 2`
   - Faye's rebate = 1000*2 − 1900 = 100.
     `faye_user_account.unsettled_quote += 100`
   - `erin_order.filled_quantity = 2`, status = PartiallyFilled
     (original 5, filled 2), **stays** in `erin_user_account.open_orders`.

   Step D — clean book. Dan's ask was fully filled → drop index 0.
   Erin's ask was only partially filled → stays. `order_book.asks =
   [(2, 950)]`. But note: the `OrderEntry` in the book does not track
   `filled_quantity`. The book just knows the order_id and price;
   the `Order` PDA carries the live remaining quantity. The next
   taker who wants to hit Erin's ask will pass `order_2` as a maker,
   and `place_order` will read its current `original_quantity -
   filled_quantity = 3`.

   Step E — pay the fee. `total_fee_quote = 4 + 1 = 5`. One CPI:
   ```
   quote_vault --[5 quote]--> fee_vault
   ```

   Step F — apply Faye's deltas. `faye_user_account.unsettled_base =
   0 + 7 = 7`. `faye_user_account.unsettled_quote = 0 + (500 + 100) =
   600`.

   Step G — rest the remainder. `taker_remaining = 0` → Faye's new
   Order is marked `Filled` immediately, not added to the book.

4. Later, each user calls `settle_funds`:
   - Dan's settle: `base_vault` loses 0 base; `quote_vault` loses
     4496 quote → Dan's quote ATA gains 4496.
   - Erin's settle: 1899 quote to Erin's ATA.
   - Faye's settle: 7 base to Faye's base ATA; 600 quote refund to
     Faye's quote ATA (unused from her 7000 lock).

5. At some point the market authority calls `withdraw_fees`:
   `fee_vault.balance = 5` → drained to authority's quote ATA.

**Post-settlement invariant check**:
- `base_vault.balance` should equal sum of remaining ask quantities =
  3 (Erin's remainder). ✓
- `quote_vault.balance` should equal sum of resting bids = 0. ✓

### 5.4 Partial fill with a remainder

Same scenario, but Faye bids at 920 (not 1000) and quantity 8.

- Fill 0: index 0 (Dan, 900), qty 5, price 900. Taker remaining 3.
- Attempt Fill 1: index 1 (Erin, 950). Crossing check: incoming bid at
  920, resting ask at 950 → `920 >= 950` is **false**. Matching
  stops.

After applying Fill 0 and the fee, `taker_remaining = 3 > 0`. The
book-capacity check runs (still fine). Faye's new Order is marked
`PartiallyFilled` (filled 5 of 8) and inserted into `order_book.bids`
at price 920. Her `open_orders` list now includes the new order_id.

Erin's ask was untouched; the book now looks like:

```
asks  [(2, 950)]        ← Erin, original 5 left
bids  [(3, 920)]        ← Faye, remaining 3
```

### 5.5 Cancel + settle round trip

Taker Gael places a bid at 910 for quantity 4 on an empty book (no
maker pairs passed). The bid rests.

- Step A (lock): `910 * 4 = 3640` quote moved from Gael's ATA to
  quote_vault. `order_book.bids = [(4, 910)]`.
- Step B–F: no fills, no fee, no maker mutations.
- Step G: `taker_remaining = 4 = quantity` → status `Open`, added
  to the book, `gael_user_account.open_orders = [4]`.

Gael decides to cancel. `cancel_order` on order_id 4:

- `remaining_quantity(order) = 4 - 0 = 4`.
- `gael_user_account.unsettled_quote += 910 * 4 = 3640`.
- `order_book.bids` cleared. `gael_user_account.open_orders = []`.
- `order.status = Cancelled`.

No tokens moved — `quote_vault.balance` still holds the 3640.

Gael calls `settle_funds`:

- `quote_vault --[3640 quote]--> gael_user_quote_account`
- `gael_user_account.unsettled_quote = 0`.

Net effect: Gael's balance sheet is exactly where it started; the
program earned nothing (no fill means no fee).

---

## 6. Full-lifecycle worked examples

Three scenarios with end-to-end numbers. Both mints are 6-decimal SPL
tokens. 1 BASE = 1 000 000 base units; 1 QUOTE = 1 000 000 quote
units. Where a number in the narrative looks like "price 900", read
that as "900 quote units per 1 base unit" (so for a 1-full-BASE trade
you'd move 900 * 1 000 000 quote units).

Market configuration:
- `fee_basis_points = 50` (0.5%)
- `tick_size = 1`
- `min_order_size = 1`
- `base_vault`, `quote_vault`, `fee_vault` all start empty.

### 6.1 A clean match: taker bid consumes a resting ask

Cast: **Maria** (market authority + Alice/Bob's broker), **Alice**
(seller), **Bob** (buyer).

1. `initialize_market` — Maria runs it. Rent for five accounts comes
   out of her wallet. Market is now `is_active`.
2. `create_user_account` — Alice and Bob each run it once.
3. Alice posts an ask: `place_order(Ask, 1000, 5)`, no
   remaining_accounts (empty book).
   - Lock: `alice_base_account --[5 base]--> base_vault`.
   - Plan: nothing to cross.
   - Rest: new Order PDA with `original_quantity = 5`, status `Open`,
     added to `order_book.asks` at index 0. `alice.open_orders = [1]`.
4. Bob posts a bid: `place_order(Bid, 1000, 5)`, with Alice's Order
   and UserAccount as remaining_accounts.
   - Lock: `bob_quote_account --[5 * 1000 = 5000 quote]-->
     quote_vault`.
   - Plan: one fill at (resting_index 0, order_id 1, qty 5, price
     1000).
   - Apply:
     - gross = 5000, fee = 5000 * 50 / 10 000 = 25, net_to_maker =
       4975.
     - `alice.unsettled_quote += 4975`
     - `bob.unsettled_base += 5`
     - Bob's rebate = 0 (he bid at the resting price exactly).
     - Alice's Order: filled 5, status Filled. Removed from
       `alice.open_orders`.
   - Clean book: drop index 0. `order_book.asks = []`.
   - Fee CPI: `quote_vault --[25 quote]--> fee_vault`.
   - Apply Bob's deltas.
   - Rest remainder: `taker_remaining = 0`, so Bob's new Order is
     marked Filled immediately, not booked.

**Balances at this point (in vault land):**
- `base_vault`: 5 base (waiting for Bob's settle).
- `quote_vault`: 4975 quote (waiting for Alice's settle). The other
  25 is now in fee_vault.
- `alice.unsettled_quote = 4975`, `alice.unsettled_base = 0`.
- `bob.unsettled_base = 5`, `bob.unsettled_quote = 0`.

5. Alice calls `settle_funds`:
   ```
   quote_vault --[4975 quote]--> alice_quote_account
   ```
   `alice.unsettled_quote = 0`.

6. Bob calls `settle_funds`:
   ```
   base_vault --[5 base]--> bob_base_account
   ```
   `bob.unsettled_base = 0`.

7. Maria calls `withdraw_fees`:
   ```
   fee_vault --[25 quote]--> maria_quote_account
   ```

**Final balance sheet (deltas from start):**
- Alice: −5 base, +4975 quote.
- Bob: +5 base, −5000 quote.
- Maria: +25 quote (minus whatever lamports she spent on rent for
  accounts).
- All three vaults empty.

### 6.2 Partial fill with remainder on the book

Cast: Alice (ask maker), Bob (bid maker, then remainder rests), Carol
(new taker).

1. `initialize_market` by Maria (same config).
2. `create_user_account` × 3.
3. Alice posts `Ask, 1000, 3`. Locks 3 base.
4. Bob posts `Bid, 1100, 10` with Alice's pair as a maker.
   - Lock: `10 * 1100 = 11_000 quote` from Bob to quote_vault.
   - Plan one fill: qty = min(10, 3) = 3, price = 1000.
   - gross = 3000, fee = 15, net_to_maker = 2985.
     - `alice.unsettled_quote += 2985`
     - `bob.unsettled_base += 3`
     - Rebate: `1100*3 − 3000 = 300` → `bob.unsettled_quote += 300`.
     - Alice's order fully filled.
   - Clean book: drop Alice's ask. `asks = []`.
   - Fee CPI: 15 quote to fee_vault.
   - `taker_remaining = 10 − 3 = 7`. Capacity OK. Bob's new Order
     marked PartiallyFilled (filled 3 of 10), added to
     `order_book.bids` at price 1100. `bob.open_orders = [2]`.

   Book state now: `asks=[], bids=[(2, 1100)]`. `quote_vault` holds
   the locked portion for Bob's remainder:
   `11000 − (3000 + 300 + 2985) = 4715`? Let's double-check: 2985 is
   *inside* quote_vault (alice's unsettled). 300 is *inside*
   quote_vault (bob's rebate unsettled). 15 went to fee_vault. 3000
   minus fee = 2985 net_to_maker sits in quote_vault waiting for
   Alice's settle. So `quote_vault.balance = 11000 − 15 = 10985`,
   composed of: alice.unsettled_quote (2985) + bob.unsettled_quote
   (300) + bob's remaining lock for the resting bid (1100 * 7 =
   7700). 2985 + 300 + 7700 = 10 985. ✓

5. Alice settles: `quote_vault --[2985]--> alice_quote_account`.
   `quote_vault = 10985 − 2985 = 8000` (= 7700 Bob-lock + 300
   Bob-rebate).
6. Carol posts `Ask, 1100, 4` with Bob's Order/UserAccount as a
   maker pair.
   - Lock: 4 base from Carol to base_vault.
   - Plan: fill at (index 0, order_id 2, qty min(4, 7) = 4, price
     1100).
   - gross = 4400, fee = 22, net_to_taker = 4378.
     - `carol.unsettled_quote += 4378`
     - `bob.unsettled_base += 4` (he's the maker-bid; base flows to
       the bid side)
     - No rebate on ask-taker side.
     - Bob's order: filled_quantity 3 → 7, status PartiallyFilled
       (still not fully filled — original 10, filled 7).
   - Clean book: Bob's book remaining = 10 − 7 = 3 > 0, so his
     entry stays. `order_book.bids = [(2, 1100)]`.
   - Fee CPI: 22 quote → fee_vault.
   - `taker_remaining = 0` → Carol's new Order marked Filled.

   Mid-state: `base_vault = 0 + 4 = 4` (from Carol's lock; was 0
   after Bob's settle made it flow — wait, no: Bob's base never
   settled yet. Let's re-check:)

   After step 4 Bob's `unsettled_base = 3` (from the 3-base fill
   against Alice). `base_vault.balance = 3 + 0 = 3` (Alice's
   original lock after the fill; asks had drained out with the
   match). After step 6, Carol added 4 base and 4 went to Bob as
   unsettled. So `base_vault.balance = 3 + 4 = 7`. `bob.unsettled_base
   = 3 + 4 = 7`.

### 6.3 Cancel round-trip

Cast: Alice (bid maker), nobody else.

1. `initialize_market`, `create_user_account(Alice)`.
2. Alice posts `Bid, 900, 10` — rests on an empty book.
   - Lock: 9000 quote from Alice to quote_vault.
   - No fills. `alice.open_orders = [1]`. `bids = [(1, 900)]`.
3. Alice reconsiders and calls `cancel_order` on her bid.
   - `remaining_quantity = 10 − 0 = 10`.
   - `alice.unsettled_quote += 900 * 10 = 9000`.
   - `bids = []`, `alice.open_orders = []`.
   - `order.status = Cancelled`.
4. Alice calls `settle_funds`:
   ```
   quote_vault --[9000 quote]--> alice_quote_account
   ```
   `alice.unsettled_quote = 0`.

Net delta: Alice is exactly where she started. The vaults are empty.
The Order account is still on chain in `Cancelled` state (one could
imagine a future instruction to reclaim its rent — see §9).

---

## 7. Safety and edge cases

### 7.1 What the program refuses to do

From [`errors.rs`](programs/clob/src/errors.rs):

| Error | When |
|---|---|
| `InvalidPrice` | `place_order` called with `price == 0` |
| `InvalidQuantity` | Reserved (not currently triggered by the handlers) |
| `OrderNotFound` | `cancel_order` failed to locate the order in the book (sanity path) |
| `MarketPaused` | `place_order` on a market with `is_active = false` (no instruction flips this today, but the field is there) |
| `Unauthorized` | `cancel_order` by someone other than the order owner |
| `OrderBookFull` | `place_order` remainder would push the book past `200` total entries |
| `TooManyOpenOrders` | User already has 20 open orders on this market |
| `InvalidTickSize` | `tick_size == 0` at init, or `price % tick_size != 0` on place |
| `BelowMinOrderSize` | `min_order_size == 0` at init, or `quantity < min_order_size` on place |
| `OrderNotCancellable` | `cancel_order` on a Filled or Cancelled order |
| `NumericalOverflow` | Any checked arithmetic returned `None` |
| `InvalidFeeBasisPoints` | `fee_basis_points > 10_000` at init |
| `InvalidFeeVault` | `market.fee_vault` on the struct does not match the passed `fee_vault` (Anchor `has_one`) |
| `MakerAccountMismatch` | Wrong number of maker accounts, wrong order, wrong market, or caller walked the book out of order |
| `MissingMakerAccounts` | `remaining_accounts.len()` not a multiple of 2 |
| `MakerOwnerMismatch` | Maker Order and UserAccount have different owners |
| `NotMarketAuthority` | `withdraw_fees` called by wrong signer |

### 7.2 Guarded design choices worth knowing

- **Full lock on place.** The handler always moves the full locked
  amount into the vault before matching. This keeps the
  vault-balance invariant simple and makes `cancel_order` / partial
  fills straightforward: the vault already has everything it could
  owe.

- **Caller supplies maker pairs.** The matching engine does not
  iterate the whole book looking for counterparties — the caller
  tells it which resting orders to cross. This is what Openbook v2
  does and it's the only way to fit the matching work within a
  transaction's account budget when the book is large. The cost is
  that an off-book client needs to read the `OrderBook` account
  first, pick the crossings, and pass the right accounts. The
  program still enforces order (price-time priority) and ownership
  on what the caller passes, so a malicious caller cannot cross a
  non-top-of-book maker to hurt someone else — they can only *fail
  to cross* orders they should have crossed, which only hurts
  themselves.

- **Matching applies at the maker's price, not the taker's.** The
  fill price is always the resting order's price. Takers that cross
  deeper into the book get price improvement, refunded to
  `unsettled_quote` (for taker bids). This is the standard CLOB
  rule.

- **Fees come out of the gross.** The maker receives `gross - fee`,
  not `gross`; the fee lives on for a while in `quote_vault` before
  being moved to `fee_vault` in one batched CPI at the end of
  `place_order`. An alternative model — the taker paying `gross +
  fee` on top of the lock — is discussed in a comment in
  `place_order.rs` and left as an exercise.

- **Unsettled balances are pure accounting.** No token physically
  moves to or from a user during matching or cancellation. Both
  events just bump `unsettled_*` counters. The user collects by
  calling `settle_funds`. This means one `place_order` call that
  crosses many makers only costs one token CPI (the fee move), not
  one-per-fill. Large orders stay within the CU budget.

- **`settle_funds` no-ops on zero.** Both legs are guarded by `if
  base_amount > 0` / `if quote_amount > 0`. Safe to schedule on a
  cron or heartbeat.

- **`withdraw_fees` no-ops on empty.** Likewise.

- **Boxed InterfaceAccounts.** Several handlers use `Box<
  InterfaceAccount<...>>` for mint/token accounts. That's a BPF
  stack-size workaround — each `InterfaceAccount` is ~1 KB on the
  stack and the Solana VM gives handlers a tight budget. Don't
  unbox these without testing the compute output size.

- **Discriminator + `has_one`.** Every state account carries an 8-
  byte discriminator that Anchor checks. `Market` has
  `has_one = fee_vault`, so the `place_order` handler can trust the
  `fee_vault` account without re-checking its mint or authority.

- **Book capacity check after matching.** The taker's remainder
  check happens at the end. A bid that clears enough asks to free
  up 3 slots can then rest its own 1-slot remainder even on a
  previously-full book — matching the "liquidity-positive" spirit
  of a CLOB.

### 7.3 Things this example does *not* do

A production CLOB would add:

- **Zero-copy OrderBook.** 100 entries per side deserialised every
  call limits both throughput and maximum book size.
- **Cancel-on-expiry / GTC vs IOC vs FOK.** All orders here are
  implicitly GTC (good 'til cancelled).
- **Post-only / reject-if-cross.** No way to guarantee your order
  will be a maker.
- **Self-trade protection.** Nothing stops a single user from
  crossing their own resting order.
- **Rent reclamation for closed orders.** `Order` accounts persist
  on chain in `Filled` or `Cancelled` state forever; a real program
  would either close them in the same instruction or provide a
  `close_order` to reclaim rent later.
- **Partial taker-funded fees.** The fee comes out of the maker's
  gross today (see `place_order.rs` comment). If you want
  maker-neutral fees, take an additional transfer from the taker's
  ATA at match time.
- **Minimum-tick for quantities.** `min_order_size` is a floor, but
  there's no "round lot" constraint.
- **Pause / admin / upgrade.** `is_active` exists but no instruction
  flips it.
- **Oracle-aware price bands.** A taker bid 10 000× higher than the
  best ask will happily sweep the book.

---

## 8. Running the tests

All tests are LiteSVM Rust integration tests under
[`programs/clob/tests/test_clob.rs`](programs/clob/tests/test_clob.rs).
They load the built `.so` via
`include_bytes!("../../../target/deploy/clob.so")`, so a build must
run first.

### Prerequisites

- Anchor 1.0.0
- Solana CLI (`solana -V`)
- Rust stable (pinned at the repo root)

### Commands

From `defi/clob/anchor/`:

```bash
# 1. Build the .so — target/deploy/clob.so
anchor build

# 2. Run the LiteSVM tests
cargo test --manifest-path programs/clob/Cargo.toml

# Or equivalently (Anchor.toml scripts.test = "cargo test"):
anchor test --skip-local-validator
```

Expected:

```
running 23 tests
test authority_can_withdraw_fees_after_match ... ok
test cancel_and_settle_bid_refunds_full_quote ... ok
test cancel_ask_credits_unsettled_base ... ok
test cancel_order_rejects_non_owner ... ok
test create_user_account_tracks_market_and_owner ... ok
test fee_vault_receives_exactly_bps_of_taker_gross ... ok
test initialize_market_rejects_oversized_fee ... ok
test initialize_market_rejects_zero_tick_size ... ok
test initialize_market_sets_market_and_order_book ... ok
test place_ask_locks_base_in_vault ... ok
test place_bid_locks_quote_in_vault ... ok
test place_order_rejects_below_min_order_size ... ok
test place_order_rejects_unaligned_tick ... ok
test place_order_rejects_zero_price ... ok
test resting_orders_at_same_price_fill_by_time_priority ... ok
test settle_funds_after_match_pays_out_both_unsettled_balances ... ok
test settle_funds_moves_unsettled_base_to_user ... ok
test taker_ask_fully_crosses_best_bid ... ok
test taker_bid_fully_crosses_best_ask ... ok
test taker_bid_gets_price_improvement_from_resting_ask ... ok
test taker_crosses_multiple_resting_orders_best_price_first ... ok
test taker_partially_filled_remainder_rests_on_book ... ok
test taker_partially_fills_resting_order_rest_stays_on_book ... ok
```

### What each test exercises

**Setup / happy path (pre-matching):**

| Test | Exercises |
|---|---|
| `initialize_market_sets_market_and_order_book` | PDA creation, vault setup, initial field values |
| `create_user_account_tracks_market_and_owner` | Per-user PDA derivation and zero-initialised counters |
| `place_bid_locks_quote_in_vault` | Fund lock on bid |
| `place_ask_locks_base_in_vault` | Fund lock on ask |
| `settle_funds_moves_unsettled_base_to_user` | Vault → user ATA transfer via market PDA signer |

**Validation:**

| Test | Exercises |
|---|---|
| `place_order_rejects_zero_price` | `price > 0` |
| `place_order_rejects_unaligned_tick` | `price % tick_size == 0` |
| `place_order_rejects_below_min_order_size` | `quantity >= min_order_size` |
| `cancel_order_rejects_non_owner` | Ownership check on cancel |
| `initialize_market_rejects_zero_tick_size` | Init constraint |
| `initialize_market_rejects_oversized_fee` | `fee_bps <= 10_000` |

**Cancel + settle flow:**

| Test | Exercises |
|---|---|
| `cancel_ask_credits_unsettled_base` | Ask cancel → `unsettled_base += remaining` |
| `cancel_and_settle_bid_refunds_full_quote` | Round trip of a Bob-style cancellation |

**Matching engine:**

| Test | Exercises |
|---|---|
| `taker_bid_fully_crosses_best_ask` | Full-fill crossing, fee routed correctly |
| `taker_ask_fully_crosses_best_bid` | Symmetric path |
| `taker_partially_fills_resting_order_rest_stays_on_book` | Resting order's `filled_quantity` updated, not removed |
| `taker_partially_filled_remainder_rests_on_book` | Taker's remainder inserted in correct price order |
| `taker_crosses_multiple_resting_orders_best_price_first` | Walks multiple makers in price priority |
| `resting_orders_at_same_price_fill_by_time_priority` | Tie-break at same price is first-in-first-out |
| `taker_bid_gets_price_improvement_from_resting_ask` | Rebate → `unsettled_quote` |
| `fee_vault_receives_exactly_bps_of_taker_gross` | Fee math in a single batched CPI |
| `authority_can_withdraw_fees_after_match` | Fee drain after fills, authority-gated |
| `settle_funds_after_match_pays_out_both_unsettled_balances` | Both legs paid in one call |

### CI note

The repo's `.github/workflows/anchor.yml` runs `anchor build` before
`anchor test` for every changed anchor project. That matters here:
the integration tests include the BPF artefact via `include_bytes!`,
so a stale or missing `.so` would break the tests. CI is already
covered.

---

## 9. Extending the program

Ordered by difficulty.

### Easy

- **Close-on-terminal `Order`.** After a `place_order` fully fills a
  maker, close its `Order` account in the same instruction and
  refund rent to the owner. Same for `cancel_order` on an `Open`
  order. Saves on-chain storage.

- **IOC flag.** Add `post_only: bool` and `ioc: bool` parameters.
  `ioc` means "match what you can and discard the remainder instead
  of resting it". `post_only` means "reject the order if it would
  cross". Both are one-line checks around the existing matching
  logic.

- **Self-trade guard.** Reject a fill where `maker_order.owner ==
  owner.key()`. Alternative: auto-cancel the maker side.

### Moderate

- **Taker-funded fees.** Pull the fee from the taker's ATA in a
  second transfer at match time, instead of netting it out of the
  maker's gross. Preserves strict "maker pays nothing" semantics.

- **Order expiry.** Add `expires_at: i64` to `Order`. In
  `place_order`, skip resting entries whose `expires_at` is past;
  add a permissionless `sweep_expired` instruction.

- **Order-book realloc.** Replace the two `Vec<OrderEntry>` with a
  pair of fixed-length arrays plus a length prefix, so the book can
  hold many more orders without paying to realloc. Keeps
  serialisation simple; avoids the 10 KB deserialisation cost per
  call.

### Harder

- **Zero-copy slabs.** Rewrite the order book as a red-black tree in
  a zero-copy account. This is what Openbook v2 and Phoenix use in
  production.

- **Event queue.** Mirror Openbook's `EventQueue` — `place_order`
  writes "fill" events, and a separate `consume_events` instruction
  processes them in batches for the maker side. Makes matching O(1)
  in CU cost regardless of the taker's depth.

- **Market-makers as CPI users.** Formalise the `remaining_accounts`
  protocol so a market-making program can call `place_order` on
  behalf of its users, pre-computing the crossings off-chain and
  rewriting the book in one transaction.

- **Cross-market swaps.** Chain two `place_order` calls (e.g.
  base→USDC then USDC→quote2) with an outer helper that routes
  through `unsettled_*` balances without a settle in between.

---

## Code layout

```
defi/clob/anchor/
├── Anchor.toml
├── Cargo.toml
├── README.md              (this file)
└── programs/clob/
    ├── Cargo.toml
    ├── src/
    │   ├── errors.rs
    │   ├── lib.rs         #[program] entry points
    │   ├── instructions/
    │   │   ├── mod.rs
    │   │   ├── initialize_market.rs
    │   │   ├── create_user_account.rs
    │   │   ├── place_order.rs        (matching engine lives here)
    │   │   ├── cancel_order.rs
    │   │   ├── settle_funds.rs
    │   │   └── withdraw_fees.rs
    │   └── state/
    │       ├── mod.rs
    │       ├── market.rs
    │       ├── order.rs
    │       ├── order_book.rs
    │       ├── user_account.rs
    │       └── matching.rs           (pure fill-planning logic)
    └── tests/
        └── test_clob.rs              LiteSVM tests
```
