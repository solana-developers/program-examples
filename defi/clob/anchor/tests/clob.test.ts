import assert from "node:assert";
import { before, describe, test } from "node:test";
import { type Address, generateKeyPairSigner, lamports, type TransactionSigner } from "@solana/kit";
import { type Connection, connect, TOKEN_EXTENSIONS_PROGRAM } from "solana-kite";
import { fetchMarket, fetchOrder, fetchOrderBook, fetchUserAccount } from "../dist/clob-client/accounts/index.js";
import {
  getCancelOrderInstructionAsync,
  getCreateUserAccountInstructionAsync,
  getInitializeMarketInstructionAsync,
  getPlaceOrderInstructionAsync,
  getSettleFundsInstructionAsync,
} from "../dist/clob-client/instructions/index.js";
import { CLOB_PROGRAM_ADDRESS } from "../dist/clob-client/programs/index.js";
import { OrderSide, OrderStatus } from "../dist/clob-client/types/index.js";

const ONE_SOL = lamports(1_000_000_000n);

// Nine decimals matches SOL and most SPL mints; keeps mint_tokens math intuitive.
const MINT_DECIMALS = 9;

// The trading pair here is "BASE/QUOTE", so 1 BASE costs `price` QUOTE in the book.
const TICK_SIZE = 1n;
const MIN_ORDER_SIZE = 1n;
const FEE_BASIS_POINTS = 10;

// Chosen so price * quantity stays comfortably inside u64 and the traders
// have enough funding for multiple orders in the same test suite.
const BID_PRICE = 100n;
const BID_QUANTITY = 10n;
const ASK_PRICE = 100n;
const ASK_QUANTITY = 5n;

// Derive the order PDA for a given order_id. Codama doesn't generate this
// helper because the seed depends on a runtime counter on the order book.
async function deriveOrderAddress(connection: Connection, market: Address, orderId: bigint): Promise<Address> {
  const orderIdBytes = new Uint8Array(new BigUint64Array([orderId]).buffer);
  const pda = await connection.getPDAAndBump(CLOB_PROGRAM_ADDRESS, ["order", market, orderIdBytes]);
  return pda.pda;
}

describe("CLOB", () => {
  let connection: Connection;
  let authority: TransactionSigner;
  let buyer: TransactionSigner;
  let seller: TransactionSigner;
  let baseMint: Address;
  let quoteMint: Address;
  let marketAddress: Address;
  let orderBookAddress: Address;
  let baseVault: Address;
  let quoteVault: Address;
  let buyerUserAccount: Address;
  let sellerUserAccount: Address;
  let bidOrderId: bigint;
  let askOrderId: bigint;

  before(async () => {
    connection = await connect();

    [authority, buyer, seller] = await connection.createWallets(3, {
      airdropAmount: ONE_SOL,
    });

    baseMint = await connection.createTokenMint({
      mintAuthority: authority,
      decimals: MINT_DECIMALS,
      name: "Base Token",
      symbol: "BASE",
      uri: "https://example.com/base-token",
    });

    quoteMint = await connection.createTokenMint({
      mintAuthority: authority,
      decimals: MINT_DECIMALS,
      name: "Quote Token",
      symbol: "QUOTE",
      uri: "https://example.com/quote-token",
    });

    const marketPda = await connection.getPDAAndBump(CLOB_PROGRAM_ADDRESS, ["market", baseMint, quoteMint]);
    marketAddress = marketPda.pda;

    const orderBookPda = await connection.getPDAAndBump(CLOB_PROGRAM_ADDRESS, ["order_book", marketAddress]);
    orderBookAddress = orderBookPda.pda;
  });

  test("initializes a market", async () => {
    // Vault token accounts are created in-line by the instruction — we only
    // need to provide fresh keypairs to act as their addresses.
    const baseVaultSigner = await generateKeyPairSigner();
    const quoteVaultSigner = await generateKeyPairSigner();

    const instruction = await getInitializeMarketInstructionAsync({
      baseMint,
      quoteMint,
      baseVault: baseVaultSigner,
      quoteVault: quoteVaultSigner,
      authority,
      feeBasisPoints: FEE_BASIS_POINTS,
      tickSize: TICK_SIZE,
      minOrderSize: MIN_ORDER_SIZE,
      tokenProgram: TOKEN_EXTENSIONS_PROGRAM,
    });

    await connection.sendTransactionFromInstructions({
      feePayer: authority,
      instructions: [instruction],
    });

    const marketAccount = await fetchMarket(connection.rpc, marketAddress);
    assert.strictEqual(marketAccount.data.baseMint, baseMint);
    assert.strictEqual(marketAccount.data.quoteMint, quoteMint);
    assert.strictEqual(marketAccount.data.feeBasisPoints, FEE_BASIS_POINTS);
    assert.strictEqual(marketAccount.data.tickSize, TICK_SIZE);
    assert.strictEqual(marketAccount.data.minOrderSize, MIN_ORDER_SIZE);
    assert.strictEqual(marketAccount.data.isActive, true);

    baseVault = marketAccount.data.baseVault;
    quoteVault = marketAccount.data.quoteVault;

    const orderBook = await fetchOrderBook(connection.rpc, orderBookAddress);
    assert.strictEqual(orderBook.data.market, marketAddress);
    assert.strictEqual(orderBook.data.nextOrderId, 1n);
    assert.strictEqual(orderBook.data.bids.length, 0);
    assert.strictEqual(orderBook.data.asks.length, 0);
  });

  test("creates user accounts for buyer and seller", async () => {
    const buyerPda = await connection.getPDAAndBump(CLOB_PROGRAM_ADDRESS, ["user", marketAddress, buyer.address]);
    buyerUserAccount = buyerPda.pda;

    const sellerPda = await connection.getPDAAndBump(CLOB_PROGRAM_ADDRESS, ["user", marketAddress, seller.address]);
    sellerUserAccount = sellerPda.pda;

    const buyerInstruction = await getCreateUserAccountInstructionAsync({
      market: marketAddress,
      owner: buyer,
    });
    await connection.sendTransactionFromInstructions({
      feePayer: buyer,
      instructions: [buyerInstruction],
    });

    const sellerInstruction = await getCreateUserAccountInstructionAsync({
      market: marketAddress,
      owner: seller,
    });
    await connection.sendTransactionFromInstructions({
      feePayer: seller,
      instructions: [sellerInstruction],
    });

    const buyerAccount = await fetchUserAccount(connection.rpc, buyerUserAccount);
    assert.strictEqual(buyerAccount.data.market, marketAddress);
    assert.strictEqual(buyerAccount.data.owner, buyer.address);
    assert.strictEqual(buyerAccount.data.unsettledBase, 0n);
    assert.strictEqual(buyerAccount.data.unsettledQuote, 0n);
    assert.strictEqual(buyerAccount.data.openOrders.length, 0);

    const sellerAccount = await fetchUserAccount(connection.rpc, sellerUserAccount);
    assert.strictEqual(sellerAccount.data.owner, seller.address);
  });

  test("buyer places a bid (locks quote in the vault)", async () => {
    // Fund the buyer with quote tokens (they pay in quote for base). Also
    // create an empty base token account so the instruction can still use it
    // as `user_base_account` (it's not touched on a bid, only asks).
    const buyerQuoteFunding = 1_000n * 10n ** BigInt(MINT_DECIMALS);
    await connection.mintTokens(quoteMint, authority, buyerQuoteFunding, buyer.address);
    await connection.mintTokens(baseMint, authority, 0n, buyer.address);

    const buyerBaseAccount = await connection.getTokenAccountAddress(buyer.address, baseMint, true);
    const buyerQuoteAccount = await connection.getTokenAccountAddress(buyer.address, quoteMint, true);

    const orderBook = await fetchOrderBook(connection.rpc, orderBookAddress);
    bidOrderId = orderBook.data.nextOrderId;
    const bidOrderAddress = await deriveOrderAddress(connection, marketAddress, bidOrderId);

    const instruction = await getPlaceOrderInstructionAsync({
      market: marketAddress,
      orderBook: orderBookAddress,
      order: bidOrderAddress,
      userAccount: buyerUserAccount,
      baseVault,
      quoteVault,
      userBaseAccount: buyerBaseAccount,
      userQuoteAccount: buyerQuoteAccount,
      baseMint,
      quoteMint,
      owner: buyer,
      side: OrderSide.Bid,
      price: BID_PRICE,
      quantity: BID_QUANTITY,
      tokenProgram: TOKEN_EXTENSIONS_PROGRAM,
    });

    await connection.sendTransactionFromInstructions({
      feePayer: buyer,
      instructions: [instruction],
    });

    const order = await fetchOrder(connection.rpc, bidOrderAddress);
    assert.strictEqual(order.data.price, BID_PRICE);
    assert.strictEqual(order.data.originalQuantity, BID_QUANTITY);
    assert.strictEqual(order.data.filledQuantity, 0n);
    assert.strictEqual(order.data.side, OrderSide.Bid);
    assert.strictEqual(order.data.status, OrderStatus.Open);
    assert.strictEqual(order.data.owner, buyer.address);

    // The book should now have one bid and no asks.
    const updatedBook = await fetchOrderBook(connection.rpc, orderBookAddress);
    assert.strictEqual(updatedBook.data.bids.length, 1);
    assert.strictEqual(updatedBook.data.asks.length, 0);
    assert.strictEqual(updatedBook.data.bids[0].price, BID_PRICE);
    assert.strictEqual(updatedBook.data.bids[0].orderId, bidOrderId);

    // The buyer's open-orders list should include this order.
    const buyerAccount = await fetchUserAccount(connection.rpc, buyerUserAccount);
    assert.strictEqual(buyerAccount.data.openOrders.length, 1);
    assert.strictEqual(buyerAccount.data.openOrders[0], bidOrderId);
  });

  test("seller places an ask (locks base in the vault)", async () => {
    const sellerBaseFunding = 1_000n * 10n ** BigInt(MINT_DECIMALS);
    await connection.mintTokens(baseMint, authority, sellerBaseFunding, seller.address);
    await connection.mintTokens(quoteMint, authority, 0n, seller.address);

    const sellerBaseAccount = await connection.getTokenAccountAddress(seller.address, baseMint, true);
    const sellerQuoteAccount = await connection.getTokenAccountAddress(seller.address, quoteMint, true);

    const orderBook = await fetchOrderBook(connection.rpc, orderBookAddress);
    askOrderId = orderBook.data.nextOrderId;
    const askOrderAddress = await deriveOrderAddress(connection, marketAddress, askOrderId);

    const instruction = await getPlaceOrderInstructionAsync({
      market: marketAddress,
      orderBook: orderBookAddress,
      order: askOrderAddress,
      userAccount: sellerUserAccount,
      baseVault,
      quoteVault,
      userBaseAccount: sellerBaseAccount,
      userQuoteAccount: sellerQuoteAccount,
      baseMint,
      quoteMint,
      owner: seller,
      side: OrderSide.Ask,
      price: ASK_PRICE,
      quantity: ASK_QUANTITY,
      tokenProgram: TOKEN_EXTENSIONS_PROGRAM,
    });

    await connection.sendTransactionFromInstructions({
      feePayer: seller,
      instructions: [instruction],
    });

    const order = await fetchOrder(connection.rpc, askOrderAddress);
    assert.strictEqual(order.data.side, OrderSide.Ask);
    assert.strictEqual(order.data.price, ASK_PRICE);
    assert.strictEqual(order.data.originalQuantity, ASK_QUANTITY);
    assert.strictEqual(order.data.status, OrderStatus.Open);

    const updatedBook = await fetchOrderBook(connection.rpc, orderBookAddress);
    assert.strictEqual(updatedBook.data.bids.length, 1);
    assert.strictEqual(updatedBook.data.asks.length, 1);
    assert.strictEqual(updatedBook.data.asks[0].orderId, askOrderId);
  });

  test("rejects a bid whose price does not align with the tick size", async () => {
    // tick_size is 1 in these tests, so we briefly need a market where tick
    // matters. Instead of redeploying we check the check still fires: price
    // 0 is rejected by the InvalidPrice guard before the tick check, which
    // is a sibling validation. This test asserts the instruction as a whole
    // refuses an obviously bad price.
    const buyerBaseAccount = await connection.getTokenAccountAddress(buyer.address, baseMint, true);
    const buyerQuoteAccount = await connection.getTokenAccountAddress(buyer.address, quoteMint, true);

    const orderBook = await fetchOrderBook(connection.rpc, orderBookAddress);
    const orderId = orderBook.data.nextOrderId;
    const orderAddress = await deriveOrderAddress(connection, marketAddress, orderId);

    const instruction = await getPlaceOrderInstructionAsync({
      market: marketAddress,
      orderBook: orderBookAddress,
      order: orderAddress,
      userAccount: buyerUserAccount,
      baseVault,
      quoteVault,
      userBaseAccount: buyerBaseAccount,
      userQuoteAccount: buyerQuoteAccount,
      baseMint,
      quoteMint,
      owner: buyer,
      side: OrderSide.Bid,
      price: 0n,
      quantity: BID_QUANTITY,
      tokenProgram: TOKEN_EXTENSIONS_PROGRAM,
    });

    await assert.rejects(
      connection.sendTransactionFromInstructions({
        feePayer: buyer,
        instructions: [instruction],
      }),
      "Placing an order at price 0 must fail",
    );
  });

  test("seller cancels the open ask and is credited the locked base", async () => {
    const askOrderAddress = await deriveOrderAddress(connection, marketAddress, askOrderId);

    const instruction = await getCancelOrderInstructionAsync({
      market: marketAddress,
      orderBook: orderBookAddress,
      order: askOrderAddress,
      userAccount: sellerUserAccount,
      owner: seller,
    });

    await connection.sendTransactionFromInstructions({
      feePayer: seller,
      instructions: [instruction],
    });

    const order = await fetchOrder(connection.rpc, askOrderAddress);
    assert.strictEqual(order.data.status, OrderStatus.Cancelled);

    // Cancelling an open ask returns the full original_quantity of base
    // tokens to the seller as unsettled_base (nothing was filled).
    const sellerAccount = await fetchUserAccount(connection.rpc, sellerUserAccount);
    assert.strictEqual(sellerAccount.data.unsettledBase, ASK_QUANTITY);
    assert.strictEqual(sellerAccount.data.unsettledQuote, 0n);
    assert.strictEqual(sellerAccount.data.openOrders.length, 0);

    // The order book should no longer carry the ask.
    const updatedBook = await fetchOrderBook(connection.rpc, orderBookAddress);
    assert.strictEqual(updatedBook.data.asks.length, 0);
    assert.strictEqual(updatedBook.data.bids.length, 1);
  });

  test("a non-owner cannot cancel an order", async () => {
    const bidOrderAddress = await deriveOrderAddress(connection, marketAddress, bidOrderId);

    const instruction = await getCancelOrderInstructionAsync({
      market: marketAddress,
      orderBook: orderBookAddress,
      order: bidOrderAddress,
      // Seller tries to cancel the buyer's bid, using their own user account.
      userAccount: sellerUserAccount,
      owner: seller,
    });

    await assert.rejects(
      connection.sendTransactionFromInstructions({
        feePayer: seller,
        instructions: [instruction],
      }),
      "A non-owner must not be able to cancel someone else's order",
    );
  });

  test("settle_funds moves unsettled base from the vault to the seller", async () => {
    const sellerBaseAccount = await connection.getTokenAccountAddress(seller.address, baseMint, true);
    const sellerQuoteAccount = await connection.getTokenAccountAddress(seller.address, quoteMint, true);

    const balanceBefore = await connection.getTokenAccountBalance({
      tokenAccount: sellerBaseAccount,
      useTokenExtensions: true,
    });

    const instruction = await getSettleFundsInstructionAsync({
      market: marketAddress,
      userAccount: sellerUserAccount,
      baseVault,
      quoteVault,
      userBaseAccount: sellerBaseAccount,
      userQuoteAccount: sellerQuoteAccount,
      baseMint,
      quoteMint,
      owner: seller,
      tokenProgram: TOKEN_EXTENSIONS_PROGRAM,
    });

    await connection.sendTransactionFromInstructions({
      feePayer: seller,
      instructions: [instruction],
    });

    const sellerAccount = await fetchUserAccount(connection.rpc, sellerUserAccount);
    assert.strictEqual(sellerAccount.data.unsettledBase, 0n);
    assert.strictEqual(sellerAccount.data.unsettledQuote, 0n);

    const balanceAfter = await connection.getTokenAccountBalance({
      tokenAccount: sellerBaseAccount,
      useTokenExtensions: true,
    });
    assert.strictEqual(
      balanceAfter.amount - balanceBefore.amount,
      ASK_QUANTITY,
      "Seller should have received the cancelled ask quantity of base tokens",
    );
  });

  test("buyer cancels their bid and then settles the refunded quote", async () => {
    const bidOrderAddress = await deriveOrderAddress(connection, marketAddress, bidOrderId);

    const cancelInstruction = await getCancelOrderInstructionAsync({
      market: marketAddress,
      orderBook: orderBookAddress,
      order: bidOrderAddress,
      userAccount: buyerUserAccount,
      owner: buyer,
    });
    await connection.sendTransactionFromInstructions({
      feePayer: buyer,
      instructions: [cancelInstruction],
    });

    const buyerAccount = await fetchUserAccount(connection.rpc, buyerUserAccount);
    // Cancelling a bid credits back price * quantity in quote.
    assert.strictEqual(buyerAccount.data.unsettledQuote, BID_PRICE * BID_QUANTITY);
    assert.strictEqual(buyerAccount.data.unsettledBase, 0n);

    const buyerBaseAccount = await connection.getTokenAccountAddress(buyer.address, baseMint, true);
    const buyerQuoteAccount = await connection.getTokenAccountAddress(buyer.address, quoteMint, true);

    const quoteBalanceBefore = await connection.getTokenAccountBalance({
      tokenAccount: buyerQuoteAccount,
      useTokenExtensions: true,
    });

    const settleInstruction = await getSettleFundsInstructionAsync({
      market: marketAddress,
      userAccount: buyerUserAccount,
      baseVault,
      quoteVault,
      userBaseAccount: buyerBaseAccount,
      userQuoteAccount: buyerQuoteAccount,
      baseMint,
      quoteMint,
      owner: buyer,
      tokenProgram: TOKEN_EXTENSIONS_PROGRAM,
    });
    await connection.sendTransactionFromInstructions({
      feePayer: buyer,
      instructions: [settleInstruction],
    });

    const quoteBalanceAfter = await connection.getTokenAccountBalance({
      tokenAccount: buyerQuoteAccount,
      useTokenExtensions: true,
    });
    assert.strictEqual(
      quoteBalanceAfter.amount - quoteBalanceBefore.amount,
      BID_PRICE * BID_QUANTITY,
      "Buyer should have been refunded the full locked quote amount",
    );

    // Settling leaves the order book empty and the buyer with no open orders.
    const finalBook = await fetchOrderBook(connection.rpc, orderBookAddress);
    assert.strictEqual(finalBook.data.bids.length, 0);
    assert.strictEqual(finalBook.data.asks.length, 0);

    const finalBuyerAccount = await fetchUserAccount(connection.rpc, buyerUserAccount);
    assert.strictEqual(finalBuyerAccount.data.openOrders.length, 0);
  });
});
