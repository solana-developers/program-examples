//! LiteSVM tests for the CLOB program.
//!
//! Covers the full lifecycle that the program supports: initialise a market,
//! create user accounts, place bids/asks (locking the appropriate vault),
//! reject invalid prices / tick-aligned prices / undersized quantities,
//! cancel orders (which credits unsettled balances), enforce cancel
//! authorisation, and settle funds out of the vaults.
//!
//! Note: this example's `place_order` does NOT cross the book. It is a
//! "book keeper" example — the matching engine is intentionally left out to
//! keep the example scoped to CLOB data structures, vault escrow, and the
//! unsettled-balance pattern. Tests therefore do not exercise crossing.

use {
    anchor_lang::{
        solana_program::{instruction::Instruction, pubkey::Pubkey, system_program},
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_kite::{
        create_associated_token_account, create_token_mint, create_wallet,
        get_token_account_balance, mint_tokens_to_token_account,
        send_transaction_from_instructions,
    },
    solana_signer::Signer,
};

// Keep test-side seeds in sync with `programs/clob/src/state/*`. Duplicated
// rather than imported so tests stay self-contained and exercise the same
// byte strings a client SDK would use.
const MARKET_SEED: &[u8] = b"market";
const ORDER_BOOK_SEED: &[u8] = b"order_book";
const ORDER_SEED: &[u8] = b"order";
const USER_ACCOUNT_SEED: &[u8] = b"user";

// Six decimals matches USDC and keeps "1 token" == 1_000_000 base units,
// which keeps the arithmetic in the assertions easy to read.
const MINT_DECIMALS: u8 = 6;

// Market parameters used across every test. `tick_size = 1` is permissive
// enough for most scenarios; a dedicated test overrides it to verify the
// tick check fires.
const FEE_BASIS_POINTS: u16 = 10;
const TICK_SIZE: u64 = 1;
const MIN_ORDER_SIZE: u64 = 1;

// Funding for each trader's token accounts. Large enough to cover every
// order placed in the tests with room to spare.
const TRADER_STARTING_BALANCE: u64 = 1_000_000_000;

// Shared order sizing — chosen so price * quantity stays well inside u64
// and the seller's ask sits at the same price as the buyer's bid (matching
// is not implemented, they just coexist in the book).
const BID_PRICE: u64 = 100;
const BID_QUANTITY: u64 = 10;
const ASK_PRICE: u64 = 100;
const ASK_QUANTITY: u64 = 5;

fn token_program_id() -> Pubkey {
    // The program accepts either SPL Token or Token-2022 via `TokenInterface`;
    // we use classic SPL Token for tests because solana-kite's helpers create
    // classic-token mints.
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        .parse()
        .unwrap()
}

fn market_pdas(program_id: &Pubkey, base_mint: &Pubkey, quote_mint: &Pubkey) -> (Pubkey, Pubkey) {
    let (market, _) = Pubkey::find_program_address(
        &[MARKET_SEED, base_mint.as_ref(), quote_mint.as_ref()],
        program_id,
    );
    let (order_book, _) =
        Pubkey::find_program_address(&[ORDER_BOOK_SEED, market.as_ref()], program_id);
    (market, order_book)
}

fn user_account_pda(program_id: &Pubkey, market: &Pubkey, owner: &Pubkey) -> Pubkey {
    let (user_account, _) = Pubkey::find_program_address(
        &[USER_ACCOUNT_SEED, market.as_ref(), owner.as_ref()],
        program_id,
    );
    user_account
}

fn order_pda(program_id: &Pubkey, market: &Pubkey, order_id: u64) -> Pubkey {
    let (order, _) = Pubkey::find_program_address(
        &[ORDER_SEED, market.as_ref(), &order_id.to_le_bytes()],
        program_id,
    );
    order
}

// ---------------------------------------------------------------------------
// Scenario: a market with a buyer and a seller, both funded in both mints.
// ---------------------------------------------------------------------------

struct Scenario {
    svm: LiteSVM,
    program_id: Pubkey,
    // `payer` funds the mint authority + ATA creations during setup but is
    // not used directly by the tests afterwards.
    #[allow(dead_code)]
    payer: Keypair,
    authority: Keypair,
    buyer: Keypair,
    seller: Keypair,
    base_mint: Pubkey,
    quote_mint: Pubkey,
    base_vault: Keypair,
    quote_vault: Keypair,
    market: Pubkey,
    order_book: Pubkey,
    buyer_base_ata: Pubkey,
    buyer_quote_ata: Pubkey,
    seller_base_ata: Pubkey,
    seller_quote_ata: Pubkey,
    buyer_user_account: Pubkey,
    seller_user_account: Pubkey,
}

fn full_setup() -> Scenario {
    let program_id = clob::id();
    let mut svm = LiteSVM::new();
    let program_bytes = include_bytes!("../../../target/deploy/clob.so");
    svm.add_program(program_id, program_bytes).unwrap();

    // 100 SOL for the payer is overkill, but rent + a few init-ATA hops add
    // up and a generous balance keeps setup logic simple.
    let payer = create_wallet(&mut svm, 100_000_000_000).unwrap();
    let authority = create_wallet(&mut svm, 10_000_000_000).unwrap();
    let buyer = create_wallet(&mut svm, 10_000_000_000).unwrap();
    let seller = create_wallet(&mut svm, 10_000_000_000).unwrap();

    let base_mint = create_token_mint(&mut svm, &authority, MINT_DECIMALS, None).unwrap();
    let quote_mint = create_token_mint(&mut svm, &authority, MINT_DECIMALS, None).unwrap();

    // Create and fund every trader's ATAs up-front so individual tests do
    // not need to worry about mint/ATA side effects, only about CLOB state.
    let buyer_base_ata =
        create_associated_token_account(&mut svm, &buyer.pubkey(), &base_mint, &payer).unwrap();
    let buyer_quote_ata =
        create_associated_token_account(&mut svm, &buyer.pubkey(), &quote_mint, &payer).unwrap();
    let seller_base_ata =
        create_associated_token_account(&mut svm, &seller.pubkey(), &base_mint, &payer).unwrap();
    let seller_quote_ata =
        create_associated_token_account(&mut svm, &seller.pubkey(), &quote_mint, &payer).unwrap();

    mint_tokens_to_token_account(
        &mut svm,
        &base_mint,
        &seller_base_ata,
        TRADER_STARTING_BALANCE,
        &authority,
    )
    .unwrap();
    mint_tokens_to_token_account(
        &mut svm,
        &quote_mint,
        &buyer_quote_ata,
        TRADER_STARTING_BALANCE,
        &authority,
    )
    .unwrap();

    let (market, order_book) = market_pdas(&program_id, &base_mint, &quote_mint);
    let buyer_user_account = user_account_pda(&program_id, &market, &buyer.pubkey());
    let seller_user_account = user_account_pda(&program_id, &market, &seller.pubkey());

    // Vaults are plain token accounts created in-line by initialize_market
    // (not PDAs). Tests generate fresh keypairs to serve as their addresses.
    let base_vault = Keypair::new();
    let quote_vault = Keypair::new();

    Scenario {
        svm,
        program_id,
        payer,
        authority,
        buyer,
        seller,
        base_mint,
        quote_mint,
        base_vault,
        quote_vault,
        market,
        order_book,
        buyer_base_ata,
        buyer_quote_ata,
        seller_base_ata,
        seller_quote_ata,
        buyer_user_account,
        seller_user_account,
    }
}

// ---------------------------------------------------------------------------
// Instruction builders — one per program entry point.
// ---------------------------------------------------------------------------

fn build_initialize_market_ix(
    sc: &Scenario,
    fee_basis_points: u16,
    tick_size: u64,
    min_order_size: u64,
) -> Instruction {
    Instruction::new_with_bytes(
        sc.program_id,
        &clob::instruction::InitializeMarket {
            fee_basis_points,
            tick_size,
            min_order_size,
        }
        .data(),
        clob::accounts::InitializeMarket {
            market: sc.market,
            order_book: sc.order_book,
            base_mint: sc.base_mint,
            quote_mint: sc.quote_mint,
            base_vault: sc.base_vault.pubkey(),
            quote_vault: sc.quote_vault.pubkey(),
            authority: sc.authority.pubkey(),
            token_program: token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    )
}

fn build_create_user_account_ix(sc: &Scenario, owner: &Pubkey) -> Instruction {
    let user_account = user_account_pda(&sc.program_id, &sc.market, owner);
    Instruction::new_with_bytes(
        sc.program_id,
        &clob::instruction::CreateUserAccount {}.data(),
        clob::accounts::CreateUserAccount {
            user_account,
            market: sc.market,
            owner: *owner,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    )
}

#[allow(clippy::too_many_arguments)]
fn build_place_order_ix(
    sc: &Scenario,
    owner: &Keypair,
    user_account: Pubkey,
    user_base_account: Pubkey,
    user_quote_account: Pubkey,
    side: clob::state::OrderSide,
    order_id: u64,
    price: u64,
    quantity: u64,
) -> Instruction {
    let order = order_pda(&sc.program_id, &sc.market, order_id);
    Instruction::new_with_bytes(
        sc.program_id,
        &clob::instruction::PlaceOrder {
            side,
            price,
            quantity,
        }
        .data(),
        clob::accounts::PlaceOrder {
            market: sc.market,
            order_book: sc.order_book,
            order,
            user_account,
            base_vault: sc.base_vault.pubkey(),
            quote_vault: sc.quote_vault.pubkey(),
            user_base_account,
            user_quote_account,
            base_mint: sc.base_mint,
            quote_mint: sc.quote_mint,
            owner: owner.pubkey(),
            token_program: token_program_id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None),
    )
}

fn build_cancel_order_ix(
    sc: &Scenario,
    owner: &Pubkey,
    user_account: Pubkey,
    order_id: u64,
) -> Instruction {
    let order = order_pda(&sc.program_id, &sc.market, order_id);
    Instruction::new_with_bytes(
        sc.program_id,
        &clob::instruction::CancelOrder {}.data(),
        clob::accounts::CancelOrder {
            market: sc.market,
            order_book: sc.order_book,
            order,
            user_account,
            owner: *owner,
        }
        .to_account_metas(None),
    )
}

fn build_settle_funds_ix(
    sc: &Scenario,
    owner: &Pubkey,
    user_account: Pubkey,
    user_base_account: Pubkey,
    user_quote_account: Pubkey,
) -> Instruction {
    Instruction::new_with_bytes(
        sc.program_id,
        &clob::instruction::SettleFunds {}.data(),
        clob::accounts::SettleFunds {
            market: sc.market,
            user_account,
            base_vault: sc.base_vault.pubkey(),
            quote_vault: sc.quote_vault.pubkey(),
            user_base_account,
            user_quote_account,
            base_mint: sc.base_mint,
            quote_mint: sc.quote_mint,
            owner: *owner,
            token_program: token_program_id(),
        }
        .to_account_metas(None),
    )
}

// Convenience: run `initialize_market` with the shared test parameters and
// both user-account creations so tests that just want a ready-to-trade
// market do not have to repeat the boilerplate.
fn initialize_market_and_users(sc: &mut Scenario) {
    let init_ix = build_initialize_market_ix(sc, FEE_BASIS_POINTS, TICK_SIZE, MIN_ORDER_SIZE);
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![init_ix],
        &[&sc.authority, &sc.base_vault, &sc.quote_vault],
        &sc.authority.pubkey(),
    )
    .unwrap();

    let buyer_ix = build_create_user_account_ix(sc, &sc.buyer.pubkey());
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![buyer_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    let seller_ix = build_create_user_account_ix(sc, &sc.seller.pubkey());
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![seller_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn initialize_market_sets_market_and_order_book() {
    let mut sc = full_setup();

    let ix = build_initialize_market_ix(&sc, FEE_BASIS_POINTS, TICK_SIZE, MIN_ORDER_SIZE);
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![ix],
        &[&sc.authority, &sc.base_vault, &sc.quote_vault],
        &sc.authority.pubkey(),
    )
    .unwrap();

    // The market PDA is owned by the program and non-empty.
    let market_account = sc.svm.get_account(&sc.market).expect("market PDA missing");
    assert_eq!(market_account.owner, sc.program_id);
    assert!(!market_account.data.is_empty());

    let order_book_account = sc
        .svm
        .get_account(&sc.order_book)
        .expect("order book PDA missing");
    assert_eq!(order_book_account.owner, sc.program_id);

    // Vaults were created with the market as authority; easiest check is
    // simply that they exist with a zero balance.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.base_vault.pubkey()).unwrap(),
        0
    );
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.quote_vault.pubkey()).unwrap(),
        0
    );
}

#[test]
fn create_user_account_tracks_market_and_owner() {
    let mut sc = full_setup();

    let init_ix = build_initialize_market_ix(&sc, FEE_BASIS_POINTS, TICK_SIZE, MIN_ORDER_SIZE);
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![init_ix],
        &[&sc.authority, &sc.base_vault, &sc.quote_vault],
        &sc.authority.pubkey(),
    )
    .unwrap();

    let create_ix = build_create_user_account_ix(&sc, &sc.buyer.pubkey());
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![create_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    let user_account = sc
        .svm
        .get_account(&sc.buyer_user_account)
        .expect("user account PDA missing");
    assert_eq!(user_account.owner, sc.program_id);
}

#[test]
fn place_bid_locks_quote_in_vault() {
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    // The first order ever placed gets id = 1 (see initialize_market.rs).
    let bid_order_id = 1u64;
    let ix = build_place_order_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        bid_order_id,
        BID_PRICE,
        BID_QUANTITY,
    );
    send_transaction_from_instructions(&mut sc.svm, vec![ix], &[&sc.buyer], &sc.buyer.pubkey())
        .unwrap();

    // A bid locks price * quantity in the quote vault.
    let locked_quote = BID_PRICE * BID_QUANTITY;
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.quote_vault.pubkey()).unwrap(),
        locked_quote
    );
    // Buyer's quote ATA dropped by exactly that.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.buyer_quote_ata).unwrap(),
        TRADER_STARTING_BALANCE - locked_quote
    );
    // Base vault untouched — bids never move base tokens.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.base_vault.pubkey()).unwrap(),
        0
    );

    // Order PDA exists and is owned by the program.
    let order_account = sc
        .svm
        .get_account(&order_pda(&sc.program_id, &sc.market, bid_order_id))
        .expect("order PDA missing");
    assert_eq!(order_account.owner, sc.program_id);
}

#[test]
fn place_ask_locks_base_in_vault() {
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    let ask_order_id = 1u64;
    let ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        ask_order_id,
        ASK_PRICE,
        ASK_QUANTITY,
    );
    send_transaction_from_instructions(&mut sc.svm, vec![ix], &[&sc.seller], &sc.seller.pubkey())
        .unwrap();

    // An ask locks `quantity` of base tokens in the base vault.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.base_vault.pubkey()).unwrap(),
        ASK_QUANTITY
    );
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.seller_base_ata).unwrap(),
        TRADER_STARTING_BALANCE - ASK_QUANTITY
    );
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.quote_vault.pubkey()).unwrap(),
        0
    );
}

#[test]
fn place_order_rejects_zero_price() {
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    let order_id = 1u64;
    let ix = build_place_order_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        order_id,
        // Price 0 trips InvalidPrice before tick-size is even considered.
        0,
        BID_QUANTITY,
    );
    let result = send_transaction_from_instructions(
        &mut sc.svm,
        vec![ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    );
    assert!(result.is_err(), "order at price 0 must be rejected");
}

#[test]
fn place_order_rejects_unaligned_tick() {
    let mut sc = full_setup();

    // Override default TICK_SIZE for this test so we can place a mis-aligned
    // price and see the tick check fire.
    let unusual_tick_size: u64 = 50;
    let init_ix =
        build_initialize_market_ix(&sc, FEE_BASIS_POINTS, unusual_tick_size, MIN_ORDER_SIZE);
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![init_ix],
        &[&sc.authority, &sc.base_vault, &sc.quote_vault],
        &sc.authority.pubkey(),
    )
    .unwrap();

    let create_ix = build_create_user_account_ix(&sc, &sc.buyer.pubkey());
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![create_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    // 75 is not a multiple of 50 — must be rejected by the tick check.
    let unaligned_price: u64 = 75;
    let ix = build_place_order_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        1,
        unaligned_price,
        BID_QUANTITY,
    );
    let result = send_transaction_from_instructions(
        &mut sc.svm,
        vec![ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    );
    assert!(
        result.is_err(),
        "unaligned price must be rejected by tick check"
    );
}

#[test]
fn place_order_rejects_below_min_order_size() {
    let mut sc = full_setup();

    // Force a higher min_order_size so we can place an order below it.
    let elevated_min_order_size: u64 = 10;
    let init_ix =
        build_initialize_market_ix(&sc, FEE_BASIS_POINTS, TICK_SIZE, elevated_min_order_size);
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![init_ix],
        &[&sc.authority, &sc.base_vault, &sc.quote_vault],
        &sc.authority.pubkey(),
    )
    .unwrap();

    let create_ix = build_create_user_account_ix(&sc, &sc.seller.pubkey());
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![create_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    let too_small_quantity: u64 = 1;
    let ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        1,
        ASK_PRICE,
        too_small_quantity,
    );
    let result = send_transaction_from_instructions(
        &mut sc.svm,
        vec![ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    );
    assert!(
        result.is_err(),
        "quantity below min_order_size must be rejected"
    );
}

#[test]
fn cancel_ask_credits_unsettled_base() {
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    // Seller places an ask, then cancels it. The full locked base should be
    // credited to unsettled_base (no settlement yet).
    let ask_order_id = 1u64;
    let place_ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        ask_order_id,
        ASK_PRICE,
        ASK_QUANTITY,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![place_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    let cancel_ix = build_cancel_order_ix(
        &sc,
        &sc.seller.pubkey(),
        sc.seller_user_account,
        ask_order_id,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![cancel_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    // Funds are still in the vault — cancel does not move tokens, it only
    // updates the unsettled balance. Settlement is a separate step.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.base_vault.pubkey()).unwrap(),
        ASK_QUANTITY
    );
    // Seller's ATA hasn't received anything back yet.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.seller_base_ata).unwrap(),
        TRADER_STARTING_BALANCE - ASK_QUANTITY
    );
}

#[test]
fn cancel_order_rejects_non_owner() {
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    // Buyer places a bid; seller tries to cancel it using their own user
    // account. The program's `order.owner == signer` check must reject.
    let bid_order_id = 1u64;
    let place_ix = build_place_order_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        bid_order_id,
        BID_PRICE,
        BID_QUANTITY,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![place_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    let attack_ix = build_cancel_order_ix(
        &sc,
        &sc.seller.pubkey(),
        sc.seller_user_account,
        bid_order_id,
    );
    let result = send_transaction_from_instructions(
        &mut sc.svm,
        vec![attack_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    );
    assert!(
        result.is_err(),
        "non-owner must not be able to cancel an order"
    );
}

#[test]
fn settle_funds_moves_unsettled_base_to_user() {
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    // Seller places + cancels an ask → credits unsettled_base.
    let ask_order_id = 1u64;
    let place_ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        ask_order_id,
        ASK_PRICE,
        ASK_QUANTITY,
    );
    let cancel_ix = build_cancel_order_ix(
        &sc,
        &sc.seller.pubkey(),
        sc.seller_user_account,
        ask_order_id,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![place_ix, cancel_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    let settle_ix = build_settle_funds_ix(
        &sc,
        &sc.seller.pubkey(),
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![settle_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    // Vault drained, seller got their base tokens back in full.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.base_vault.pubkey()).unwrap(),
        0
    );
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.seller_base_ata).unwrap(),
        TRADER_STARTING_BALANCE
    );
}

#[test]
fn cancel_and_settle_bid_refunds_full_quote() {
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    let bid_order_id = 1u64;
    let place_ix = build_place_order_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        bid_order_id,
        BID_PRICE,
        BID_QUANTITY,
    );
    let cancel_ix = build_cancel_order_ix(
        &sc,
        &sc.buyer.pubkey(),
        sc.buyer_user_account,
        bid_order_id,
    );
    let settle_ix = build_settle_funds_ix(
        &sc,
        &sc.buyer.pubkey(),
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![place_ix, cancel_ix, settle_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    // Vault drained, buyer got the full price*quantity of quote back.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.quote_vault.pubkey()).unwrap(),
        0
    );
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.buyer_quote_ata).unwrap(),
        TRADER_STARTING_BALANCE
    );
}

#[test]
fn initialize_market_rejects_zero_tick_size() {
    let mut sc = full_setup();

    let zero_tick_size: u64 = 0;
    let ix = build_initialize_market_ix(&sc, FEE_BASIS_POINTS, zero_tick_size, MIN_ORDER_SIZE);
    let result = send_transaction_from_instructions(
        &mut sc.svm,
        vec![ix],
        &[&sc.authority, &sc.base_vault, &sc.quote_vault],
        &sc.authority.pubkey(),
    );
    assert!(result.is_err(), "tick_size == 0 must be rejected");
}

#[test]
fn initialize_market_rejects_oversized_fee() {
    let mut sc = full_setup();

    // 10_000 bps == 100% is the cap; anything higher must fail.
    let over_cap_fee_basis_points: u16 = 10_001;
    let ix = build_initialize_market_ix(
        &sc,
        over_cap_fee_basis_points,
        TICK_SIZE,
        MIN_ORDER_SIZE,
    );
    let result = send_transaction_from_instructions(
        &mut sc.svm,
        vec![ix],
        &[&sc.authority, &sc.base_vault, &sc.quote_vault],
        &sc.authority.pubkey(),
    );
    assert!(
        result.is_err(),
        "fee_basis_points above 10_000 must be rejected"
    );
}
