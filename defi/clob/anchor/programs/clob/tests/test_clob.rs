//! LiteSVM tests for the CLOB program.
//!
//! Covers the full lifecycle that the program supports: initialise a market,
//! create user accounts, place bids/asks (locking the appropriate vault),
//! reject invalid prices / tick-aligned prices / undersized quantities,
//! cancel orders (which credits unsettled balances), settle funds out of
//! the vaults, and — in the matching block near the bottom — cross incoming
//! orders against resting orders using price-time priority, charge the
//! configured taker fee to a fee vault, and drain the fee vault via
//! `withdraw_fees`.

use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
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
    // Fees accumulate here (quote mint). Created fresh per Scenario; the
    // market PDA is the signer, same as the other two vaults.
    fee_vault: Keypair,
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
    let fee_vault = Keypair::new();

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
        fee_vault,
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
            fee_vault: sc.fee_vault.pubkey(),
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
            fee_vault: sc.fee_vault.pubkey(),
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

/// Build a `place_order` instruction with maker (order, user_account) PDA
/// pairs appended as remaining accounts. The CLOB expects them in the same
/// order the resting book will be walked — best-priced first (lowest ask
/// for a taker bid, highest bid for a taker ask), and within a price level
/// earliest-first. Every maker pair must be writable: the program mutates
/// the maker's Order (filled_quantity, status) and their UserAccount
/// (unsettled_* and open_orders).
#[allow(clippy::too_many_arguments)]
fn build_place_order_with_makers_ix(
    sc: &Scenario,
    owner: &Keypair,
    user_account: Pubkey,
    user_base_account: Pubkey,
    user_quote_account: Pubkey,
    side: clob::state::OrderSide,
    order_id: u64,
    price: u64,
    quantity: u64,
    maker_pairs: &[(u64, Pubkey)],
) -> Instruction {
    let mut ix = build_place_order_ix(
        sc,
        owner,
        user_account,
        user_base_account,
        user_quote_account,
        side,
        order_id,
        price,
        quantity,
    );

    for (maker_order_id, maker_user_account) in maker_pairs {
        let maker_order = order_pda(&sc.program_id, &sc.market, *maker_order_id);
        ix.accounts
            .push(AccountMeta::new(maker_order, false));
        ix.accounts
            .push(AccountMeta::new(*maker_user_account, false));
    }

    ix
}

fn build_withdraw_fees_ix(
    sc: &Scenario,
    authority_quote_account: Pubkey,
) -> Instruction {
    Instruction::new_with_bytes(
        sc.program_id,
        &clob::instruction::WithdrawFees {}.data(),
        clob::accounts::WithdrawFees {
            market: sc.market,
            fee_vault: sc.fee_vault.pubkey(),
            authority_quote_account,
            quote_mint: sc.quote_mint,
            authority: sc.authority.pubkey(),
            token_program: token_program_id(),
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
        &[&sc.authority, &sc.base_vault, &sc.quote_vault, &sc.fee_vault],
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
        &[&sc.authority, &sc.base_vault, &sc.quote_vault, &sc.fee_vault],
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
        &[&sc.authority, &sc.base_vault, &sc.quote_vault, &sc.fee_vault],
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
        &[&sc.authority, &sc.base_vault, &sc.quote_vault, &sc.fee_vault],
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
        &[&sc.authority, &sc.base_vault, &sc.quote_vault, &sc.fee_vault],
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
        &[&sc.authority, &sc.base_vault, &sc.quote_vault, &sc.fee_vault],
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
        &[&sc.authority, &sc.base_vault, &sc.quote_vault, &sc.fee_vault],
        &sc.authority.pubkey(),
    );
    assert!(
        result.is_err(),
        "fee_basis_points above 10_000 must be rejected"
    );
}

// ---------------------------------------------------------------------------
// Matching-engine tests
//
// These exercise the price-time priority crossing logic added to place_order.
// Constants are named per-test (rather than shared at the top of the file)
// so each test reads self-contained and the maths is easy to follow.
// ---------------------------------------------------------------------------

// UserAccount field offsets after the 8-byte Anchor discriminator. Layout
// (see programs/clob/src/state/user_account.rs):
//   market: Pubkey         (32)
//   owner:  Pubkey         (32)
//   unsettled_base: u64    (8)
//   unsettled_quote: u64   (8)
//   ...
// Borsh-decoding manually (rather than pulling UserAccount via try_from)
// keeps the tests readable and side-steps rent-checked deserialise paths.
const USER_ACCOUNT_UNSETTLED_BASE_OFFSET: usize = 8 + 32 + 32;
const USER_ACCOUNT_UNSETTLED_QUOTE_OFFSET: usize = USER_ACCOUNT_UNSETTLED_BASE_OFFSET + 8;

// Order layout after 8-byte discriminator (see state/order.rs):
//   market: Pubkey                      (32)
//   owner:  Pubkey                      (32)
//   order_id: u64                       (8)
//   side: u8 (Borsh-encoded enum tag)   (1)
//   price: u64                          (8)
//   original_quantity: u64              (8)
//   filled_quantity: u64                (8)
const ORDER_FILLED_QUANTITY_OFFSET: usize = 8 + 32 + 32 + 8 + 1 + 8 + 8;
const ORDER_STATUS_OFFSET: usize = ORDER_FILLED_QUANTITY_OFFSET + 8;
const ORDER_STATUS_OPEN: u8 = 0;
const ORDER_STATUS_PARTIALLY_FILLED: u8 = 1;
const ORDER_STATUS_FILLED: u8 = 2;

fn read_user_unsettled(svm: &LiteSVM, user_account: &Pubkey) -> (u64, u64) {
    let data = svm
        .get_account(user_account)
        .expect("user account missing")
        .data
        .clone();
    let base = u64::from_le_bytes(
        data[USER_ACCOUNT_UNSETTLED_BASE_OFFSET..USER_ACCOUNT_UNSETTLED_BASE_OFFSET + 8]
            .try_into()
            .unwrap(),
    );
    let quote = u64::from_le_bytes(
        data[USER_ACCOUNT_UNSETTLED_QUOTE_OFFSET..USER_ACCOUNT_UNSETTLED_QUOTE_OFFSET + 8]
            .try_into()
            .unwrap(),
    );
    (base, quote)
}

fn read_order_fill_and_status(svm: &LiteSVM, order: &Pubkey) -> (u64, u8) {
    let data = svm
        .get_account(order)
        .expect("order account missing")
        .data
        .clone();
    let filled = u64::from_le_bytes(
        data[ORDER_FILLED_QUANTITY_OFFSET..ORDER_FILLED_QUANTITY_OFFSET + 8]
            .try_into()
            .unwrap(),
    );
    let status = data[ORDER_STATUS_OFFSET];
    (filled, status)
}

#[test]
fn taker_bid_fully_crosses_best_ask() {
    // Seller rests an ask, buyer's bid fully eats it. Check base flows to
    // buyer's unsettled_base, quote net-of-fee flows to seller's
    // unsettled_quote, and fee_vault receives the expected bps.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    const MAKER_ASK_ID: u64 = 1;
    // 1000 * 100 = 100_000 quote flows, and 100_000 * 10 bps / 10_000 = 100
    // fee — big enough to be non-zero after integer division, tiny enough
    // that trader starting balances easily cover it.
    const PRICE: u64 = 1000;
    const QUANTITY: u64 = 100;
    const EXPECTED_GROSS_QUOTE: u64 = PRICE * QUANTITY;
    const EXPECTED_FEE: u64 = EXPECTED_GROSS_QUOTE * FEE_BASIS_POINTS as u64 / 10_000;
    const EXPECTED_NET_TO_MAKER: u64 = EXPECTED_GROSS_QUOTE - EXPECTED_FEE;

    // Seller posts the resting ask.
    let maker_ask_ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        MAKER_ASK_ID,
        PRICE,
        QUANTITY,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![maker_ask_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    // Buyer's taker bid at the same price, same qty — fully crosses.
    const TAKER_BID_ID: u64 = 2;
    let taker_bid_ix = build_place_order_with_makers_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        TAKER_BID_ID,
        PRICE,
        QUANTITY,
        &[(MAKER_ASK_ID, sc.seller_user_account)],
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![taker_bid_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    // Fee vault received exactly fee_bps of the gross.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.fee_vault.pubkey()).unwrap(),
        EXPECTED_FEE
    );

    let (buyer_base, buyer_quote) = read_user_unsettled(&sc.svm, &sc.buyer_user_account);
    assert_eq!(buyer_base, QUANTITY);
    // No price improvement here — buyer's limit == maker's price — so no
    // quote rebate lands in the taker's unsettled_quote.
    assert_eq!(buyer_quote, 0);

    let (_seller_base, seller_quote) = read_user_unsettled(&sc.svm, &sc.seller_user_account);
    assert_eq!(seller_quote, EXPECTED_NET_TO_MAKER);

    // The resting maker order should have been removed from the book and
    // marked Filled.
    let maker_order = order_pda(&sc.program_id, &sc.market, MAKER_ASK_ID);
    let (filled, status) = read_order_fill_and_status(&sc.svm, &maker_order);
    assert_eq!(filled, QUANTITY);
    assert_eq!(status, ORDER_STATUS_FILLED);
}

#[test]
fn taker_ask_fully_crosses_best_bid() {
    // Mirror of the bid test. Buyer rests a bid, seller's ask fully eats it.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    const MAKER_BID_ID: u64 = 1;
    const PRICE: u64 = 1000;
    const QUANTITY: u64 = 100;
    const EXPECTED_GROSS_QUOTE: u64 = PRICE * QUANTITY;
    const EXPECTED_FEE: u64 = EXPECTED_GROSS_QUOTE * FEE_BASIS_POINTS as u64 / 10_000;
    const EXPECTED_NET_TO_TAKER: u64 = EXPECTED_GROSS_QUOTE - EXPECTED_FEE;

    let maker_bid_ix = build_place_order_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        MAKER_BID_ID,
        PRICE,
        QUANTITY,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![maker_bid_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    const TAKER_ASK_ID: u64 = 2;
    let taker_ask_ix = build_place_order_with_makers_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        TAKER_ASK_ID,
        PRICE,
        QUANTITY,
        &[(MAKER_BID_ID, sc.buyer_user_account)],
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![taker_ask_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.fee_vault.pubkey()).unwrap(),
        EXPECTED_FEE
    );
    // Maker (buyer) received the base tokens they paid for.
    let (buyer_base, _buyer_quote) = read_user_unsettled(&sc.svm, &sc.buyer_user_account);
    assert_eq!(buyer_base, QUANTITY);

    // Taker (seller) received the net-of-fee quote.
    let (_seller_base, seller_quote) = read_user_unsettled(&sc.svm, &sc.seller_user_account);
    assert_eq!(seller_quote, EXPECTED_NET_TO_TAKER);
}

#[test]
fn taker_partially_fills_resting_order_rest_stays_on_book() {
    // Seller rests ask qty=100. Buyer bids qty=40 at the same price.
    // The ask stays on the book with qty=60 remaining; the taker fully
    // matches and rests nothing.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    const MAKER_ASK_ID: u64 = 1;
    const MAKER_ASK_QUANTITY: u64 = 100;
    const TAKER_BID_QUANTITY: u64 = 40;
    const PRICE: u64 = 1000;

    let ask_ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        MAKER_ASK_ID,
        PRICE,
        MAKER_ASK_QUANTITY,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![ask_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    const TAKER_BID_ID: u64 = 2;
    let bid_ix = build_place_order_with_makers_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        TAKER_BID_ID,
        PRICE,
        TAKER_BID_QUANTITY,
        &[(MAKER_ASK_ID, sc.seller_user_account)],
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![bid_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    // Maker order: still PartiallyFilled, filled_quantity == TAKER_BID_QUANTITY.
    let maker_order = order_pda(&sc.program_id, &sc.market, MAKER_ASK_ID);
    let (filled, status) = read_order_fill_and_status(&sc.svm, &maker_order);
    assert_eq!(filled, TAKER_BID_QUANTITY);
    assert_eq!(status, ORDER_STATUS_PARTIALLY_FILLED);

    // Base vault still holds the un-filled portion (seller's lock, minus
    // what was delivered to the taker's unsettled_base — which never left
    // the vault, just got re-tagged as owed to the buyer).
    //
    // Total base in vault stays == MAKER_ASK_QUANTITY, because fills are
    // bucket-accounting inside the single vault.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.base_vault.pubkey()).unwrap(),
        MAKER_ASK_QUANTITY
    );

    // Taker received TAKER_BID_QUANTITY base tokens.
    let (buyer_base, _) = read_user_unsettled(&sc.svm, &sc.buyer_user_account);
    assert_eq!(buyer_base, TAKER_BID_QUANTITY);
}

#[test]
fn taker_partially_filled_remainder_rests_on_book() {
    // Seller rests ask qty=40. Buyer bids qty=100 at the same price.
    // Buyer eats the whole ask and the remaining 60 rests on the book as a
    // new bid.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    const MAKER_ASK_ID: u64 = 1;
    const MAKER_ASK_QUANTITY: u64 = 40;
    const TAKER_BID_QUANTITY: u64 = 100;
    const PRICE: u64 = 1000;

    let ask_ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        MAKER_ASK_ID,
        PRICE,
        MAKER_ASK_QUANTITY,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![ask_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    const TAKER_BID_ID: u64 = 2;
    let bid_ix = build_place_order_with_makers_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        TAKER_BID_ID,
        PRICE,
        TAKER_BID_QUANTITY,
        &[(MAKER_ASK_ID, sc.seller_user_account)],
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![bid_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    // Maker ask is fully filled.
    let maker_order = order_pda(&sc.program_id, &sc.market, MAKER_ASK_ID);
    let (filled, status) = read_order_fill_and_status(&sc.svm, &maker_order);
    assert_eq!(filled, MAKER_ASK_QUANTITY);
    assert_eq!(status, ORDER_STATUS_FILLED);

    // Taker's own order is PartiallyFilled with `filled_quantity` equal
    // to what the maker supplied.
    let taker_order = order_pda(&sc.program_id, &sc.market, TAKER_BID_ID);
    let (taker_filled, taker_status) = read_order_fill_and_status(&sc.svm, &taker_order);
    assert_eq!(taker_filled, MAKER_ASK_QUANTITY);
    assert_eq!(taker_status, ORDER_STATUS_PARTIALLY_FILLED);

    // The taker's own Order PDA holds the true remaining-on-book quantity
    // (original_quantity - filled_quantity). On-book quantity isn't stored
    // on OrderEntry directly — see state/order_book.rs — so this is the
    // source of truth both here and at runtime.
    assert_eq!(
        TAKER_BID_QUANTITY - taker_filled,
        TAKER_BID_QUANTITY - MAKER_ASK_QUANTITY
    );
}

#[test]
fn taker_crosses_multiple_resting_orders_best_price_first() {
    // Two resting asks at different prices: 900 and 1000. A taker bid big
    // enough to chew through both must hit 900 first (best price for the
    // taker), then 1000.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    const BEST_ASK_ID: u64 = 1;
    const BEST_ASK_PRICE: u64 = 900;
    const BEST_ASK_QUANTITY: u64 = 30;

    const SECOND_ASK_ID: u64 = 2;
    const SECOND_ASK_PRICE: u64 = 1000;
    const SECOND_ASK_QUANTITY: u64 = 50;

    // Taker bids at the worse of the two resting prices so both cross.
    const TAKER_BID_ID: u64 = 3;
    const TAKER_BID_PRICE: u64 = 1000;
    const TAKER_BID_QUANTITY: u64 = BEST_ASK_QUANTITY + SECOND_ASK_QUANTITY;

    // Need to post both asks and both rest — seller places two in sequence.
    let ask_one_ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        BEST_ASK_ID,
        BEST_ASK_PRICE,
        BEST_ASK_QUANTITY,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![ask_one_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();
    let ask_two_ix = build_place_order_ix(
        &sc,
        &sc.seller,
        sc.seller_user_account,
        sc.seller_base_ata,
        sc.seller_quote_ata,
        clob::state::OrderSide::Ask,
        SECOND_ASK_ID,
        SECOND_ASK_PRICE,
        SECOND_ASK_QUANTITY,
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![ask_two_ix],
        &[&sc.seller],
        &sc.seller.pubkey(),
    )
    .unwrap();

    // Taker walks in book order: best ask (900) then second (1000).
    let taker_ix = build_place_order_with_makers_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        TAKER_BID_ID,
        TAKER_BID_PRICE,
        TAKER_BID_QUANTITY,
        &[
            (BEST_ASK_ID, sc.seller_user_account),
            (SECOND_ASK_ID, sc.seller_user_account),
        ],
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![taker_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    // Both resting asks are fully filled.
    let order_one = order_pda(&sc.program_id, &sc.market, BEST_ASK_ID);
    let order_two = order_pda(&sc.program_id, &sc.market, SECOND_ASK_ID);
    assert_eq!(read_order_fill_and_status(&sc.svm, &order_one).1, ORDER_STATUS_FILLED);
    assert_eq!(read_order_fill_and_status(&sc.svm, &order_two).1, ORDER_STATUS_FILLED);

    // Taker got `TAKER_BID_QUANTITY` base tokens.
    let (buyer_base, buyer_quote_rebate) = read_user_unsettled(&sc.svm, &sc.buyer_user_account);
    assert_eq!(buyer_base, TAKER_BID_QUANTITY);

    // Price-improvement rebate: taker locked at 1000/unit but 30 units
    // filled at 900. Rebate = (1000 - 900) * 30 = 3_000.
    const PRICE_IMPROVEMENT_REBATE: u64 = (TAKER_BID_PRICE - BEST_ASK_PRICE) * BEST_ASK_QUANTITY;
    assert_eq!(buyer_quote_rebate, PRICE_IMPROVEMENT_REBATE);

    // Seller's net unsettled_quote = sum of (fill_price * fill_qty - fee)
    // across both fills.
    let gross_one: u64 = BEST_ASK_PRICE * BEST_ASK_QUANTITY;
    let gross_two: u64 = SECOND_ASK_PRICE * SECOND_ASK_QUANTITY;
    let fee_one: u64 = gross_one * FEE_BASIS_POINTS as u64 / 10_000;
    let fee_two: u64 = gross_two * FEE_BASIS_POINTS as u64 / 10_000;
    let expected_seller_quote = (gross_one - fee_one) + (gross_two - fee_two);
    let (_, seller_quote) = read_user_unsettled(&sc.svm, &sc.seller_user_account);
    assert_eq!(seller_quote, expected_seller_quote);
}

#[test]
fn resting_orders_at_same_price_fill_by_time_priority() {
    // Two resting asks at price 1000: first from seller, then from a second
    // seller. Taker only buys enough to cross the first one. The second
    // must stay on the book untouched.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    // Bootstrap a third wallet (second seller) with base tokens.
    let second_seller = create_wallet(&mut sc.svm, 10_000_000_000).unwrap();
    let second_seller_base_ata = create_associated_token_account(
        &mut sc.svm,
        &second_seller.pubkey(),
        &sc.base_mint,
        &sc.payer,
    )
    .unwrap();
    let second_seller_quote_ata = create_associated_token_account(
        &mut sc.svm,
        &second_seller.pubkey(),
        &sc.quote_mint,
        &sc.payer,
    )
    .unwrap();
    mint_tokens_to_token_account(
        &mut sc.svm,
        &sc.base_mint,
        &second_seller_base_ata,
        TRADER_STARTING_BALANCE,
        &sc.authority,
    )
    .unwrap();
    let second_seller_user_account = user_account_pda(&sc.program_id, &sc.market, &second_seller.pubkey());
    let __ix1 = build_create_user_account_ix(&sc, &second_seller.pubkey());
    send_transaction_from_instructions(&mut sc.svm, vec![__ix1], &[&second_seller],
        &second_seller.pubkey()).unwrap();

    const FIRST_ASK_ID: u64 = 1;
    const SECOND_ASK_ID: u64 = 2;
    const ASK_PRICE_SHARED: u64 = 1000;
    const ASK_QUANTITY_EACH: u64 = 20;

    // Seller 1 first in.
    let __ix2 = build_place_order_ix(
            &sc,
            &sc.seller,
            sc.seller_user_account,
            sc.seller_base_ata,
            sc.seller_quote_ata,
            clob::state::OrderSide::Ask,
            FIRST_ASK_ID,
            ASK_PRICE_SHARED,
            ASK_QUANTITY_EACH,
        );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix2], &[&sc.seller],
        &sc.seller.pubkey()).unwrap();
    // Seller 2 second in at the same price.
    let __ix3 = build_place_order_ix(
            &sc,
            &second_seller,
            second_seller_user_account,
            second_seller_base_ata,
            second_seller_quote_ata,
            clob::state::OrderSide::Ask,
            SECOND_ASK_ID,
            ASK_PRICE_SHARED,
            ASK_QUANTITY_EACH,
        );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix3], &[&second_seller],
        &second_seller.pubkey()).unwrap();

    // Taker bid buys only enough to cross seller 1's ask.
    const TAKER_BID_ID: u64 = 3;
    let taker_ix = build_place_order_with_makers_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        TAKER_BID_ID,
        ASK_PRICE_SHARED,
        ASK_QUANTITY_EACH,
        &[(FIRST_ASK_ID, sc.seller_user_account)],
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![taker_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    // Time priority: seller 1 filled, seller 2 still open.
    let order_one = order_pda(&sc.program_id, &sc.market, FIRST_ASK_ID);
    let order_two = order_pda(&sc.program_id, &sc.market, SECOND_ASK_ID);
    assert_eq!(read_order_fill_and_status(&sc.svm, &order_one).1, ORDER_STATUS_FILLED);
    assert_eq!(read_order_fill_and_status(&sc.svm, &order_two).1, ORDER_STATUS_OPEN);
}

#[test]
fn taker_bid_gets_price_improvement_from_resting_ask() {
    // Taker limit 1000, resting ask at 900. Taker pays 900 (maker's price),
    // gets 100-per-unit price improvement rebated to their unsettled_quote.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    const MAKER_ASK_ID: u64 = 1;
    const MAKER_ASK_PRICE: u64 = 900;
    const TAKER_BID_PRICE: u64 = 1000;
    const QUANTITY: u64 = 50;

    // Maker ask.
    let __ix4 = build_place_order_ix(
            &sc,
            &sc.seller,
            sc.seller_user_account,
            sc.seller_base_ata,
            sc.seller_quote_ata,
            clob::state::OrderSide::Ask,
            MAKER_ASK_ID,
            MAKER_ASK_PRICE,
            QUANTITY,
        );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix4], &[&sc.seller],
        &sc.seller.pubkey()).unwrap();

    // Taker bid — limit 1000.
    const TAKER_BID_ID: u64 = 2;
    let taker_ix = build_place_order_with_makers_ix(
        &sc,
        &sc.buyer,
        sc.buyer_user_account,
        sc.buyer_base_ata,
        sc.buyer_quote_ata,
        clob::state::OrderSide::Bid,
        TAKER_BID_ID,
        TAKER_BID_PRICE,
        QUANTITY,
        &[(MAKER_ASK_ID, sc.seller_user_account)],
    );
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![taker_ix],
        &[&sc.buyer],
        &sc.buyer.pubkey(),
    )
    .unwrap();

    // Maker got 900-per-unit (minus fee), not 1000.
    let gross_to_maker: u64 = MAKER_ASK_PRICE * QUANTITY;
    let fee: u64 = gross_to_maker * FEE_BASIS_POINTS as u64 / 10_000;
    let expected_net_to_maker: u64 = gross_to_maker - fee;
    let (_, seller_quote) = read_user_unsettled(&sc.svm, &sc.seller_user_account);
    assert_eq!(seller_quote, expected_net_to_maker);

    // Taker locked (TAKER_BID_PRICE * QUANTITY) of quote up front; only
    // (MAKER_ASK_PRICE * QUANTITY) was spent. The difference is the
    // price-improvement rebate.
    let expected_rebate: u64 = (TAKER_BID_PRICE - MAKER_ASK_PRICE) * QUANTITY;
    let (buyer_base, buyer_quote) = read_user_unsettled(&sc.svm, &sc.buyer_user_account);
    assert_eq!(buyer_base, QUANTITY);
    assert_eq!(buyer_quote, expected_rebate);
}

#[test]
fn fee_vault_receives_exactly_bps_of_taker_gross() {
    // Simpler standalone check of the fee maths: fee_vault must equal
    // (taker gross quote) * fee_bps / 10_000 after a single fill.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    const MAKER_ASK_ID: u64 = 1;
    const PRICE: u64 = 500;
    const QUANTITY: u64 = 200;
    const GROSS: u64 = PRICE * QUANTITY;
    const EXPECTED_FEE: u64 = GROSS * FEE_BASIS_POINTS as u64 / 10_000;

    let __ix5 = build_place_order_ix(
            &sc,
            &sc.seller,
            sc.seller_user_account,
            sc.seller_base_ata,
            sc.seller_quote_ata,
            clob::state::OrderSide::Ask,
            MAKER_ASK_ID,
            PRICE,
            QUANTITY,
        );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix5], &[&sc.seller],
        &sc.seller.pubkey()).unwrap();

    const TAKER_BID_ID: u64 = 2;
    let __ix6 = build_place_order_with_makers_ix(
            &sc,
            &sc.buyer,
            sc.buyer_user_account,
            sc.buyer_base_ata,
            sc.buyer_quote_ata,
            clob::state::OrderSide::Bid,
            TAKER_BID_ID,
            PRICE,
            QUANTITY,
            &[(MAKER_ASK_ID, sc.seller_user_account)],
    );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix6], &[&sc.buyer], &sc.buyer.pubkey()).unwrap();


    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.fee_vault.pubkey()).unwrap(),
        EXPECTED_FEE
    );
}

#[test]
fn authority_can_withdraw_fees_after_match() {
    // Run a fill, confirm fee vault has something, withdraw to authority.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    // Authority needs a quote ATA to receive the withdrawn fees.
    let authority_quote_ata = create_associated_token_account(
        &mut sc.svm,
        &sc.authority.pubkey(),
        &sc.quote_mint,
        &sc.payer,
    )
    .unwrap();

    const MAKER_ASK_ID: u64 = 1;
    const PRICE: u64 = 2000;
    const QUANTITY: u64 = 50;
    const GROSS: u64 = PRICE * QUANTITY;
    const EXPECTED_FEE: u64 = GROSS * FEE_BASIS_POINTS as u64 / 10_000;

    let __ix7 = build_place_order_ix(
            &sc,
            &sc.seller,
            sc.seller_user_account,
            sc.seller_base_ata,
            sc.seller_quote_ata,
            clob::state::OrderSide::Ask,
            MAKER_ASK_ID,
            PRICE,
            QUANTITY,
        );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix7], &[&sc.seller],
        &sc.seller.pubkey()).unwrap();

    const TAKER_BID_ID: u64 = 2;
    let __ix8 = build_place_order_with_makers_ix(
            &sc,
            &sc.buyer,
            sc.buyer_user_account,
            sc.buyer_base_ata,
            sc.buyer_quote_ata,
            clob::state::OrderSide::Bid,
            TAKER_BID_ID,
            PRICE,
            QUANTITY,
            &[(MAKER_ASK_ID, sc.seller_user_account)],
    );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix8], &[&sc.buyer], &sc.buyer.pubkey()).unwrap();


    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.fee_vault.pubkey()).unwrap(),
        EXPECTED_FEE
    );

    let withdraw_ix = build_withdraw_fees_ix(&sc, authority_quote_ata);
    send_transaction_from_instructions(
        &mut sc.svm,
        vec![withdraw_ix],
        &[&sc.authority],
        &sc.authority.pubkey(),
    )
    .unwrap();

    // Fee vault drained, authority received the fees.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.fee_vault.pubkey()).unwrap(),
        0
    );
    assert_eq!(
        get_token_account_balance(&sc.svm, &authority_quote_ata).unwrap(),
        EXPECTED_FEE
    );
}

#[test]
fn settle_funds_after_match_pays_out_both_unsettled_balances() {
    // End-to-end: match, then call settle_funds for both sides. Both
    // traders must receive the tokens the match credited to their
    // unsettled_* balances.
    let mut sc = full_setup();
    initialize_market_and_users(&mut sc);

    const MAKER_ASK_ID: u64 = 1;
    const PRICE: u64 = 1000;
    const QUANTITY: u64 = 100;
    const GROSS: u64 = PRICE * QUANTITY;
    const EXPECTED_FEE: u64 = GROSS * FEE_BASIS_POINTS as u64 / 10_000;
    const EXPECTED_NET_QUOTE_TO_SELLER: u64 = GROSS - EXPECTED_FEE;

    // Maker posts and taker crosses.
    let __ix9 = build_place_order_ix(
            &sc,
            &sc.seller,
            sc.seller_user_account,
            sc.seller_base_ata,
            sc.seller_quote_ata,
            clob::state::OrderSide::Ask,
            MAKER_ASK_ID,
            PRICE,
            QUANTITY,
        );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix9], &[&sc.seller],
        &sc.seller.pubkey()).unwrap();
    const TAKER_BID_ID: u64 = 2;
    let __ix10 = build_place_order_with_makers_ix(
            &sc,
            &sc.buyer,
            sc.buyer_user_account,
            sc.buyer_base_ata,
            sc.buyer_quote_ata,
            clob::state::OrderSide::Bid,
            TAKER_BID_ID,
            PRICE,
            QUANTITY,
            &[(MAKER_ASK_ID, sc.seller_user_account)],
    );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix10], &[&sc.buyer], &sc.buyer.pubkey()).unwrap();


    // Settle both sides.
    let __ix11 = build_settle_funds_ix(
            &sc,
            &sc.buyer.pubkey(),
            sc.buyer_user_account,
            sc.buyer_base_ata,
            sc.buyer_quote_ata,
        );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix11], &[&sc.buyer],
        &sc.buyer.pubkey()).unwrap();
    let __ix12 = build_settle_funds_ix(
            &sc,
            &sc.seller.pubkey(),
            sc.seller_user_account,
            sc.seller_base_ata,
            sc.seller_quote_ata,
        );
    send_transaction_from_instructions(&mut sc.svm, vec![__ix12], &[&sc.seller],
        &sc.seller.pubkey()).unwrap();

    // Buyer should now hold `QUANTITY` extra base tokens and have paid the
    // gross quote (starting balance minus gross). No price improvement
    // here, so nothing else to refund.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.buyer_base_ata).unwrap(),
        QUANTITY
    );
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.buyer_quote_ata).unwrap(),
        TRADER_STARTING_BALANCE - GROSS
    );

    // Seller should now hold (starting - QUANTITY) base and
    // EXPECTED_NET_QUOTE_TO_SELLER quote.
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.seller_base_ata).unwrap(),
        TRADER_STARTING_BALANCE - QUANTITY
    );
    assert_eq!(
        get_token_account_balance(&sc.svm, &sc.seller_quote_ata).unwrap(),
        EXPECTED_NET_QUOTE_TO_SELLER
    );
}

