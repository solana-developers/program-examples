use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::ErrorCode;
use crate::state::{
    add_ask, add_bid, add_open_order, plan_fills, remove_open_order, Market, Order, OrderBook,
    OrderSide, OrderStatus, UserAccount, MARKET_SEED, MAX_ORDERS_PER_SIDE, ORDER_BOOK_SEED,
    ORDER_SEED, USER_ACCOUNT_SEED,
};

// Mirror of UserAccount.open_orders max_len. Kept as a constant so the
// PlaceOrder check reads clearly and the limit is documented in one place.
const MAX_OPEN_ORDERS_PER_USER: usize = 20;

// Basis-points denominator. 10_000 bps == 100% — standard in TradFi and CEXes.
const BASIS_POINTS_DENOMINATOR: u128 = 10_000;

// Remaining accounts are passed in groups of 2 per resting order we intend
// to cross: [maker_order, maker_user_account]. We keep it at 2 (instead of
// also threading the maker's ATAs) because fills land in the maker's
// unsettled_* balance — the maker drains them later via settle_funds. This
// mirrors how Openbook v2 works and keeps the per-fill account footprint
// small.
const ACCOUNTS_PER_MAKER: usize = 2;

pub fn handle_place_order<'info>(
    context: Context<'info, PlaceOrder<'info>>,
    side: OrderSide,
    price: u64,
    quantity: u64,
) -> Result<()> {
    let market = &context.accounts.market;

    require!(market.is_active, ErrorCode::MarketPaused);
    require!(price > 0, ErrorCode::InvalidPrice);
    require!(price % market.tick_size == 0, ErrorCode::InvalidTickSize);
    require!(
        quantity >= market.min_order_size,
        ErrorCode::BelowMinOrderSize
    );

    require!(
        context.accounts.user_account.open_orders.len() < MAX_OPEN_ORDERS_PER_USER,
        ErrorCode::TooManyOpenOrders
    );

    // Lock up the funds the order would need if filled. Bids lock quote
    // (price * quantity); asks lock base (quantity). This always happens —
    // matching consumes from the locked pot (already in the vault), and any
    // unmatched remainder rests as a maker order with its lock still in place.
    let (source_account, mint_account_info, decimals, transfer_amount, destination_vault) =
        match side {
            OrderSide::Bid => (
                context.accounts.user_quote_account.to_account_info(),
                context.accounts.quote_mint.to_account_info(),
                context.accounts.quote_mint.decimals,
                price
                    .checked_mul(quantity)
                    .ok_or(ErrorCode::NumericalOverflow)?,
                context.accounts.quote_vault.to_account_info(),
            ),
            OrderSide::Ask => (
                context.accounts.user_base_account.to_account_info(),
                context.accounts.base_mint.to_account_info(),
                context.accounts.base_mint.decimals,
                quantity,
                context.accounts.base_vault.to_account_info(),
            ),
        };

    transfer_checked(
        CpiContext::new(
            context.accounts.token_program.key(),
            TransferChecked {
                from: source_account,
                mint: mint_account_info,
                to: destination_vault,
                authority: context.accounts.owner.to_account_info(),
            },
        ),
        transfer_amount,
        decimals,
    )?;

    // ---------------------------------------------------------------
    // Matching
    // ---------------------------------------------------------------
    //
    // Caller passes, in the transaction's remaining_accounts slot, pairs of
    // (maker_order, maker_user_account) in the same order the taker expects
    // to cross them (best-priced first, then time priority at a tie). We
    // deserialise them up front so we can both plan fills (quantities come
    // from the live Order accounts) and mutate them below.
    let maker_accounts = &context.remaining_accounts;
    require!(
        maker_accounts.len() % ACCOUNTS_PER_MAKER == 0,
        ErrorCode::MissingMakerAccounts
    );
    let maker_pair_count = maker_accounts.len() / ACCOUNTS_PER_MAKER;

    // Parallel Vec of resting-side quantities-remaining, one per book entry,
    // so the matching planner can decide fill sizes without touching the
    // Order accounts itself. Defaults to 0 for entries the caller didn't
    // pass in — those get skipped by the planner (they would also have been
    // unreachable given price-time priority ordering).
    let order_book = &mut context.accounts.order_book;
    // Note: we don't reject yet if the book is "full" (bids + asks ==
    // 2 * MAX_ORDERS_PER_SIDE). A taker that fully crosses removes resting
    // orders before needing to add its own, so it's legitimate even on a
    // full book. We re-check below, *after* matching, right before adding
    // any remainder to the book.

    let resting_entries = match side {
        OrderSide::Bid => &order_book.asks,
        OrderSide::Ask => &order_book.bids,
    };

    let mut resting_quantities: Vec<u64> = vec![0u64; resting_entries.len()];
    // Track which maker_pair index (if any) each resting book slot maps to.
    // We only need this for slots the caller actually supplied — entries
    // beyond the supplied set can't be crossed in this transaction.
    for maker_pair_index in 0..maker_pair_count {
        let maker_order_info = &maker_accounts[maker_pair_index * ACCOUNTS_PER_MAKER];
        let maker_order = Account::<Order>::try_from(maker_order_info)?;

        // Find the corresponding book entry. The caller is expected to pass
        // makers in price-time priority order, but we don't trust that —
        // we look up by order_id and reject mismatches.
        let book_position = resting_entries
            .iter()
            .position(|entry| entry.order_id == maker_order.order_id)
            .ok_or(ErrorCode::MakerAccountMismatch)?;

        require!(maker_order.market == market.key(), ErrorCode::MakerAccountMismatch);
        resting_quantities[book_position] =
            maker_order.original_quantity.saturating_sub(maker_order.filled_quantity);

        // We also want the maker_pair_index to line up with the book slot
        // so after planning we can fetch the right (order, user_account)
        // pair. But the planner walks book slots in order, not maker_pair
        // order. To keep it simple, we require the caller to pass makers
        // starting at book slot 0 and going in book order — which is the
        // natural way to walk anyway. Enforce that here:
        require!(
            book_position == maker_pair_index,
            ErrorCode::MakerAccountMismatch
        );
    }

    let (fills, taker_remaining) = plan_fills(
        order_book,
        &resting_quantities,
        side,
        price,
        quantity,
    );

    // Accumulate taker's accounting deltas so we only touch the taker's
    // UserAccount once. base_received counts base tokens the taker gains;
    // quote_rebate is quote the taker locked but doesn't need to spend
    // (price improvement between their limit and the resting maker's
    // price), refunded to unsettled_quote for bids.
    let taker_user_account = &mut context.accounts.user_account;
    let mut taker_base_received: u64 = 0;
    let mut taker_quote_rebate: u64 = 0;
    let mut taker_quote_received: u64 = 0;
    // Total fee to move from vault (quote side) to fee_vault. Aggregating
    // into one CPI at the end halves the CU cost vs one CPI per fill.
    let mut total_fee_quote: u64 = 0;

    for fill in &fills {
        // Maker-side mutations: find the maker_pair for this fill.
        let maker_pair_index = fill.resting_index;
        let maker_order_info = &maker_accounts[maker_pair_index * ACCOUNTS_PER_MAKER];
        let maker_user_info = &maker_accounts[maker_pair_index * ACCOUNTS_PER_MAKER + 1];

        let mut maker_order = Account::<Order>::try_from(maker_order_info)?;
        let mut maker_user_account = Account::<UserAccount>::try_from(maker_user_info)?;

        require!(
            maker_order.owner == maker_user_account.owner,
            ErrorCode::MakerOwnerMismatch
        );
        require!(
            maker_user_account.market == market.key(),
            ErrorCode::MakerAccountMismatch
        );

        // Fee model (simple, maker-funded, no extra taker deposit):
        //
        //   gross  = fill_price * fill_quantity (quote tokens per fill)
        //   fee    = gross * fee_bps / 10_000   (rounded down)
        //   maker gets gross - fee,
        //   fee_vault gets fee,
        //   taker pays 'gross' net (out of their pre-locked quote).
        //
        // Strictly "makers pay nothing" would require the taker to bring
        // (gross + fee) which means pulling more from the taker's ATA on
        // every fill — a per-fill CPI that inflates CU cost and account
        // lists. Real CLOBs (Openbook v2, Phoenix) use a similar deduct-
        // from-gross pattern for simplicity; the fee can be thought of as
        // the maker pricing their ask a fraction higher to cover it. Swap
        // to a taker-funded model by adding a second transfer_checked from
        // the taker's ATA to fee_vault if you need strict maker-neutral
        // fees.
        let gross_quote: u64 = fill
            .fill_price
            .checked_mul(fill.fill_quantity)
            .ok_or(ErrorCode::NumericalOverflow)?;

        let fee_quote: u64 = (gross_quote as u128)
            .checked_mul(market.fee_basis_points as u128)
            .ok_or(ErrorCode::NumericalOverflow)?
            .checked_div(BASIS_POINTS_DENOMINATOR)
            .ok_or(ErrorCode::NumericalOverflow)?
            .try_into()
            .map_err(|_| error!(ErrorCode::NumericalOverflow))?;

        match side {
            // Taker Bid, resting Ask. Taker pays quote, gets base.
            OrderSide::Bid => {
                // Net quote flowing to the maker after the protocol fee
                // (see fee-model comment above).
                let net_quote_to_maker = gross_quote
                    .checked_sub(fee_quote)
                    .ok_or(ErrorCode::NumericalOverflow)?;
                maker_user_account.unsettled_quote = maker_user_account
                    .unsettled_quote
                    .checked_add(net_quote_to_maker)
                    .ok_or(ErrorCode::NumericalOverflow)?;

                taker_base_received = taker_base_received
                    .checked_add(fill.fill_quantity)
                    .ok_or(ErrorCode::NumericalOverflow)?;

                // Price improvement: taker locked (price * quantity) but
                // only needs (fill_price * fill_quantity) for this fill.
                // Refund the difference to the taker's unsettled_quote.
                let locked_for_this_fill: u64 = price
                    .checked_mul(fill.fill_quantity)
                    .ok_or(ErrorCode::NumericalOverflow)?;
                let rebate: u64 = locked_for_this_fill
                    .checked_sub(gross_quote)
                    .ok_or(ErrorCode::NumericalOverflow)?;
                taker_quote_rebate = taker_quote_rebate
                    .checked_add(rebate)
                    .ok_or(ErrorCode::NumericalOverflow)?;
            }
            // Taker Ask, resting Bid. Taker gives base, gets quote.
            OrderSide::Ask => {
                // Maker (resting bidder) receives base.
                maker_user_account.unsettled_base = maker_user_account
                    .unsettled_base
                    .checked_add(fill.fill_quantity)
                    .ok_or(ErrorCode::NumericalOverflow)?;

                // Maker bid locked (bid_price * bid_qty) of quote up front
                // — so fill_price * fill_quantity of that locked pool now
                // flows to the taker as quote received, minus the taker
                // fee. No rebate to the maker (they locked exactly what
                // they're spending).
                let net_quote_to_taker = gross_quote
                    .checked_sub(fee_quote)
                    .ok_or(ErrorCode::NumericalOverflow)?;
                taker_quote_received = taker_quote_received
                    .checked_add(net_quote_to_taker)
                    .ok_or(ErrorCode::NumericalOverflow)?;
            }
        }

        total_fee_quote = total_fee_quote
            .checked_add(fee_quote)
            .ok_or(ErrorCode::NumericalOverflow)?;

        // Update the maker Order: bump filled_quantity, flip status.
        maker_order.filled_quantity = maker_order
            .filled_quantity
            .checked_add(fill.fill_quantity)
            .ok_or(ErrorCode::NumericalOverflow)?;

        let maker_fully_filled =
            maker_order.filled_quantity >= maker_order.original_quantity;
        maker_order.status = if maker_fully_filled {
            OrderStatus::Filled
        } else {
            OrderStatus::PartiallyFilled
        };

        // If the resting order is fully filled, drop it from the maker's
        // open_orders. Book removal happens in a second pass below (we
        // collect the order_ids now and remove them in reverse order so
        // indexes stay valid).
        if maker_fully_filled {
            remove_open_order(&mut maker_user_account, maker_order.order_id);
        }

        // Persist the mutations back to the account datas — exit() runs
        // Anchor's realloc/serialise + discriminator check path.
        maker_order.exit(context.program_id)?;
        maker_user_account.exit(context.program_id)?;
    }

    // Remove fully-filled resting orders from the book. Descend so indexes
    // don't shift under us.
    let mut fully_filled_indexes: Vec<usize> = fills
        .iter()
        .filter(|fill| {
            // We know the maker's quantity from resting_quantities; if the
            // fill equals that, they're fully filled.
            resting_quantities[fill.resting_index] == fill.fill_quantity
        })
        .map(|fill| fill.resting_index)
        .collect();
    fully_filled_indexes.sort_unstable();
    fully_filled_indexes.dedup();

    let resting_mut: &mut Vec<crate::state::OrderEntry> = match side {
        OrderSide::Bid => &mut order_book.asks,
        OrderSide::Ask => &mut order_book.bids,
    };
    for index in fully_filled_indexes.iter().rev() {
        resting_mut.remove(*index);
    }

    // Move accumulated fee from quote_vault → fee_vault (one CPI signed
    // by the market PDA).
    if total_fee_quote > 0 {
        let market_bump = [market.bump];
        let signer_seeds: [&[u8]; 4] = [
            MARKET_SEED,
            market.base_mint.as_ref(),
            market.quote_mint.as_ref(),
            &market_bump,
        ];
        let signer_seeds = &[&signer_seeds[..]];

        transfer_checked(
            CpiContext::new_with_signer(
                context.accounts.token_program.key(),
                TransferChecked {
                    from: context.accounts.quote_vault.to_account_info(),
                    mint: context.accounts.quote_mint.to_account_info(),
                    to: context.accounts.fee_vault.to_account_info(),
                    authority: market.to_account_info(),
                },
                signer_seeds,
            ),
            total_fee_quote,
            context.accounts.quote_mint.decimals,
        )?;
    }

    // Apply taker accounting deltas in a single mutation.
    taker_user_account.unsettled_base = taker_user_account
        .unsettled_base
        .checked_add(taker_base_received)
        .ok_or(ErrorCode::NumericalOverflow)?;
    taker_user_account.unsettled_quote = taker_user_account
        .unsettled_quote
        .checked_add(taker_quote_rebate)
        .ok_or(ErrorCode::NumericalOverflow)?
        .checked_add(taker_quote_received)
        .ok_or(ErrorCode::NumericalOverflow)?;

    // ---------------------------------------------------------------
    // Any remainder becomes a maker order — init the Order PDA, add to
    // the book, add to the owner's open_orders.
    //
    // If the taker was fully matched we still init the Order account
    // (Anchor already allocated rent for it in this instruction) but mark
    // it Filled immediately. That keeps the account valid for downstream
    // indexers that may have read the next_order_id before the transaction.
    // ---------------------------------------------------------------
    let order_id = order_book.next_order_id;
    order_book.next_order_id = order_book.next_order_id.saturating_add(1);

    let order = &mut context.accounts.order;
    order.market = market.key();
    order.owner = context.accounts.owner.key();
    order.order_id = order_id;
    order.side = side;
    order.price = price;
    order.original_quantity = quantity;
    order.filled_quantity = quantity.saturating_sub(taker_remaining);
    order.timestamp = Clock::get()?.unix_timestamp;
    order.bump = context.bumps.order;

    if taker_remaining == 0 {
        // Taker fully matched — nothing rests on the book.
        order.status = OrderStatus::Filled;
    } else {
        // Check book capacity only now — a taker that removed resting
        // orders above may have freed space even on a previously-full
        // book.
        require!(
            order_book.bids.len() + order_book.asks.len() < MAX_ORDERS_PER_SIDE * 2,
            ErrorCode::OrderBookFull
        );

        order.status = if taker_remaining < quantity {
            OrderStatus::PartiallyFilled
        } else {
            OrderStatus::Open
        };
        match side {
            OrderSide::Bid => {
                add_bid(order_book, order_id, price, context.accounts.owner.key())
            }
            OrderSide::Ask => {
                add_ask(order_book, order_id, price, context.accounts.owner.key())
            }
        }
        add_open_order(taker_user_account, order_id);
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(side: OrderSide, price: u64, quantity: u64)]
pub struct PlaceOrder<'info> {
    #[account(
        mut,
        has_one = fee_vault @ ErrorCode::InvalidFeeVault,
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [ORDER_BOOK_SEED, market.key().as_ref()],
        bump = order_book.bump
    )]
    pub order_book: Account<'info, OrderBook>,

    #[account(
        init,
        payer = owner,
        space = Order::DISCRIMINATOR.len() + Order::INIT_SPACE,
        seeds = [
            ORDER_SEED,
            market.key().as_ref(),
            order_book.next_order_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub order: Account<'info, Order>,

    #[account(
        mut,
        seeds = [USER_ACCOUNT_SEED, market.key().as_ref(), owner.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    // InterfaceAccount on the stack is ~1 KB each; with 7 of them this struct
    // blows the 4 KB stack-offset limit on BPF. Boxing moves each to the heap.
    #[account(mut)]
    pub base_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    // Taker fees are routed here. Constrained via `has_one = fee_vault` on
    // `market` above so the program can trust it without re-checking.
    #[account(mut)]
    pub fee_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user_base_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user_quote_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub base_mint: Box<InterfaceAccount<'info, Mint>>,

    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
