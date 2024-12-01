import { randomBytes } from 'node:crypto';
import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { BN, type Program } from '@coral-xyz/anchor';
import {
  MINT_SIZE,
  TOKEN_2022_PROGRAM_ID,
  type TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
} from '@solana/spl-token';
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, type TransactionInstruction } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { SecureTokenEscrowProgram } from '../target/types/secure_token_escrow_program';

import { confirmTransaction, makeKeypairs } from '@solana-developers/helpers';

/**
 * Test Configuration and Constants
 */
const TOKEN_PROGRAM: typeof TOKEN_2022_PROGRAM_ID | typeof TOKEN_PROGRAM_ID = TOKEN_2022_PROGRAM_ID;
const IDL = require('../target/idl/secure_token_escrow_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

/**
 * Helper function to generate random BN for escrow identifiers
 */
const generateRandomIdentifier = (size = 8) => {
  return new BN(randomBytes(size));
};

describe('Secure Token Escrow Program Tests', async () => {
  const context = await startAnchor('', [{ name: 'escrow', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const connection = provider.connection;
  const program = new anchor.Program<SecureTokenEscrowProgram>(IDL, provider);

  // Account registry for test cases
  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM,
  };

  // Create test participants and token mints
  const [maker, taker, offeredTokenMint, requestedTokenMint] = makeKeypairs(4);

  before('Initialize test environment with accounts and tokens', async () => {
    // Generate Associated Token Accounts for both participants
    const [makerOfferedTokenAccount, makerRequestedTokenAccount, takerOfferedTokenAccount, takerRequestedTokenAccount] = [maker, taker].flatMap(
      (keypair) =>
        [offeredTokenMint, requestedTokenMint].map((mint) => getAssociatedTokenAddressSync(mint.publicKey, keypair.publicKey, false, TOKEN_PROGRAM)),
    );

    // Setup initial account funding and token mints
    const minimumLamports = await getMinimumBalanceForRentExemptMint(connection);

    // Create SOL transfer instructions
    const fundAccountInstructions: Array<TransactionInstruction> = [maker, taker].map((account) =>
      SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: account.publicKey,
        lamports: 10 * LAMPORTS_PER_SOL,
      }),
    );

    // Create mint account instructions
    const createMintInstructions: Array<TransactionInstruction> = [offeredTokenMint, requestedTokenMint].map((mint) =>
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: mint.publicKey,
        lamports: minimumLamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM,
      }),
    );

    // Setup token minting and ATA creation instructions
    const setupTokenInstructions: Array<TransactionInstruction> = [
      {
        mint: offeredTokenMint.publicKey,
        authority: maker.publicKey,
        ata: makerOfferedTokenAccount,
      },
      {
        mint: requestedTokenMint.publicKey,
        authority: taker.publicKey,
        ata: takerRequestedTokenAccount,
      },
    ].flatMap((mintDetails) => [
      createInitializeMint2Instruction(mintDetails.mint, 6, mintDetails.authority, null, TOKEN_PROGRAM),
      createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, mintDetails.ata, mintDetails.authority, mintDetails.mint, TOKEN_PROGRAM),
      createMintToInstruction(mintDetails.mint, mintDetails.ata, mintDetails.authority, 1_000_000_000, [], TOKEN_PROGRAM),
    ]);

    // Combine all instructions into a single transaction
    const tx = new Transaction();
    tx.instructions = [...fundAccountInstructions, ...createMintInstructions, ...setupTokenInstructions];

    await provider.sendAndConfirm(tx, [offeredTokenMint, requestedTokenMint, maker, taker]);

    // Register accounts for test use
    accounts.maker = maker.publicKey;
    accounts.taker = taker.publicKey;
    accounts.offeredTokenMint = offeredTokenMint.publicKey;
    accounts.makerOfferedTokenAccount = makerOfferedTokenAccount;
    accounts.takerOfferedTokenAccount = takerOfferedTokenAccount;
    accounts.requestedTokenMint = requestedTokenMint.publicKey;
    accounts.makerRequestedTokenAccount = makerRequestedTokenAccount;
    accounts.takerRequestedTokenAccount = takerRequestedTokenAccount;
  });

  // Test constants
  const offeredTokenAmount = new BN(1_000_000);
  const requestedTokenAmount = new BN(1_000_000);

  /**
   * Helper function to create a new exchange offer
   */
  const createExchangeOffer = async () => {
    const escrowIdentifier = generateRandomIdentifier();

    // Generate PDAs for escrow and vault
    const [escrowState] = PublicKey.findProgramAddressSync(
      [Buffer.from('escrow'), accounts.maker.toBuffer(), escrowIdentifier.toArrayLike(Buffer, 'le', 8)],
      program.programId,
    );

    const escrowVault = getAssociatedTokenAddressSync(accounts.offeredTokenMint, escrowState, true, TOKEN_PROGRAM);

    accounts.escrowState = escrowState;
    accounts.escrowVault = escrowVault;

    const transactionSignature = await program.methods
      .createTokenExchangeOffer(escrowIdentifier, offeredTokenAmount, requestedTokenAmount)
      .accounts({ ...accounts })
      .signers([maker])
      .rpc();

    await confirmTransaction(connection, transactionSignature);

    // Verify vault balance
    const vaultBalance = await connection.getTokenAccountBalance(escrowVault);
    assert(new BN(vaultBalance.value.amount).eq(offeredTokenAmount));

    // Verify escrow state
    const escrowAccount = await program.account.escrow.fetch(escrowState);
    assert(escrowAccount.maker.equals(maker.publicKey));
    assert(escrowAccount.token_mint_a.equals(accounts.offeredTokenMint));
    assert(escrowAccount.token_mint_b.equals(accounts.requestedTokenMint));
    assert(escrowAccount.token_b_wanted_amount.eq(requestedTokenAmount));
  };

  /**
   * Helper function to accept an exchange offer
   */
  const acceptExchangeOffer = async () => {
    const transactionSignature = await program.methods
      .acceptTokenExchangeOffer()
      .accounts({ ...accounts })
      .signers([taker])
      .rpc();

    await confirmTransaction(connection, transactionSignature);

    // Verify taker received offered tokens
    const takerBalance = await connection.getTokenAccountBalance(accounts.takerOfferedTokenAccount);
    assert(new BN(takerBalance.value.amount).eq(offeredTokenAmount));

    // Verify maker received requested tokens
    const makerBalance = await connection.getTokenAccountBalance(accounts.makerRequestedTokenAccount);
    assert(new BN(makerBalance.value.amount).eq(requestedTokenAmount));
  };

  it('Should successfully create token exchange offer and transfer tokens to vault', async () => {
    await createExchangeOffer();
  });

  it('Should successfully complete token exchange when offer is accepted', async () => {
    await acceptExchangeOffer();
  });
});
