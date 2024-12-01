import {
  Account,
  AssociatedTokenAccount,
  Mint,
  Pubkey,
  type Result,
  Seeds,
  Signer,
  SystemAccount,
  TokenAccount,
  TokenProgram,
  UncheckedAccount,
  i64,
  u8,
  u64,
} from '@solanaturbine/poseidon';

/**
 * SecureTokenEscrowProgram
 *
 * A secure escrow system for trustless token exchanges on Solana blockchain.
 * This program facilitates peer-to-peer token trades by:
 * 1. Securing offered tokens in a program-controlled vault
 * 2. Managing the exchange of different token types
 * 3. Ensuring atomic settlement of trades
 * 4. Protecting both maker and taker interests
 */
export default class SecureTokenEscrowProgram {
  static PROGRAM_ID = new Pubkey('4JTogV8LakXjvx49uoRmzowWJ6mxtaFBksFk3kNGUdKW');

  /**
   * Creates a new token exchange offer in the escrow system
   *
   * @param maker - The account initiating the token exchange offer
   * @param offeredTokenMint - The mint of the token being offered
   * @param requestedTokenMint - The mint of the token being requested
   * @param makerOfferedTokenAccount - Maker's ATA for the offered token
   * @param escrowVault - Token account to hold the escrowed tokens
   * @param escrowAuthority - PDA with authority over escrow operations
   * @param escrowState - Account storing the escrow details
   * @param offeredTokenAmount - Amount of tokens being offered
   * @param requestedTokenAmount - Amount of tokens requested in exchange
   * @param escrowIdentifier - Unique identifier for this escrow
   * @returns Result indicating success or failure
   */
  createTokenExchangeOffer(
    maker: Signer,
    offeredTokenMint: Mint,
    requestedTokenMint: Mint,
    makerOfferedTokenAccount: AssociatedTokenAccount,
    escrowVault: TokenAccount,
    escrowAuthority: UncheckedAccount,
    escrowState: Escrow,
    offeredTokenAmount: u64,
    requestedTokenAmount: u64,
    escrowIdentifier: u64,
  ): Result {
    // Derive maker's token account PDA
    makerOfferedTokenAccount.derive(offeredTokenMint, maker.key);

    // Derive escrow authority PDA
    escrowAuthority.derive(['auth']);

    // Initialize vault PDA for holding escrowed tokens
    escrowVault.derive(['vault', escrowState.key], offeredTokenMint, escrowAuthority.key).init();

    // Initialize escrow state account with unique identifier
    escrowState.derive(['escrow', maker.key, escrowIdentifier.toBytes()]).init();

    // Store bump seeds for future PDA derivation
    escrowState.authBump = escrowAuthority.getBump();
    escrowState.vaultBump = escrowVault.getBump();
    escrowState.escrowBump = escrowState.getBump();

    // Set escrow parameters
    escrowState.maker = maker.key;
    escrowState.token_mint_a = offeredTokenMint.key;
    escrowState.token_mint_b = requestedTokenMint.key;
    escrowState.token_b_wanted_amount = requestedTokenAmount;
    escrowState.id = escrowIdentifier;

    // Transfer offered tokens to escrow vault
    TokenProgram.transfer(
      makerOfferedTokenAccount, // Source account
      escrowVault, // Destination vault
      maker, // Transfer authority
      offeredTokenAmount, // Amount to escrow
    );
  }

  /**
   * Accepts and completes an existing token exchange offer
   *
   * @param taker - The account accepting the exchange offer
   * @param maker - The original offer creator's account
   * @param makerRequestedTokenAccount - Maker's ATA for receiving requested tokens
   * @param takerOfferedTokenAccount - Taker's ATA for sending requested tokens
   * @param takerReceiveTokenAccount - Taker's ATA for receiving escrowed tokens
   * @param offeredTokenMint - Mint of the escrowed token
   * @param requestedTokenMint - Mint of the requested token
   * @param escrowAuthority - PDA with authority over escrow operations
   * @param escrowVault - Token account holding the escrowed tokens
   * @param escrowState - Account storing the escrow details
   * @returns Result indicating success or failure
   */
  acceptTokenExchangeOffer(
    taker: Signer,
    maker: SystemAccount,
    makerRequestedTokenAccount: AssociatedTokenAccount,
    takerOfferedTokenAccount: AssociatedTokenAccount,
    takerReceiveTokenAccount: AssociatedTokenAccount,
    offeredTokenMint: Mint,
    requestedTokenMint: Mint,
    escrowAuthority: UncheckedAccount,
    escrowVault: TokenAccount,
    escrowState: Escrow,
  ): Result {
    // Initialize taker's token accounts if they don't exist
    takerOfferedTokenAccount.derive(offeredTokenMint, taker.key).initIfNeeded();
    takerReceiveTokenAccount.derive(offeredTokenMint, taker.key).initIfNeeded();

    // Derive maker's token account
    makerRequestedTokenAccount.derive(offeredTokenMint, maker.key);

    // Verify and close escrow state account
    escrowState.derive(['escrow', maker.key, escrowState.id.toBytes()]).has([maker, offeredTokenMint, requestedTokenMint]).close(maker);

    // Derive escrow PDAs
    escrowAuthority.derive(['auth']);
    escrowVault.derive(['vault', escrowState.key], offeredTokenMint, escrowAuthority.key);

    // Execute token transfers atomically

    // First transfer: Taker sends requested tokens to maker
    TokenProgram.transfer(takerOfferedTokenAccount, makerRequestedTokenAccount, taker, escrowState.token_b_wanted_amount);

    // Second transfer: Release escrowed tokens to taker
    const authSeeds: Seeds = ['auth', escrowState.authBump.toBytes()];
    TokenProgram.transfer(escrowVault, takerReceiveTokenAccount, escrowAuthority, escrowState.token_b_wanted_amount, authSeeds);
  }
}

/**
 * Interface defining the state of an escrow exchange
 * Extends the base Account class to inherit Solana account properties
 */
export interface Escrow extends Account {
  maker: Pubkey; // Address of the offer creator
  token_mint_a: Pubkey; // Mint address of token being offered
  token_mint_b: Pubkey; // Mint address of token being requested
  token_b_wanted_amount: u64; // Amount of tokens requested in exchange
  escrowBump: u8; // Bump seed for escrow PDA
  id: u64; // Unique identifier for this escrow
  authBump: u8; // Bump seed for authority PDA
  vaultBump: u8; // Bump seed for vault PDA
}
