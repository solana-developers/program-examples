import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { BankrunProvider, startAnchor } from 'anchor-bankrun';
import { assert } from 'chai';
import { Counter } from '../target/types/counter';

// Import the IDL (Interface Description Language) file for the Counter program
const IDL = require('../target/idl/counter.json');

/**
 * Test suite for the Counter program
 * Tests basic functionality including initialization and increment operations
 */
describe('counter', () => {
  // Test environment variables
  let provider: BankrunProvider;
  let program: Program<Counter>;
  let authority: Keypair;

  /**
   * Set up a fresh test environment before each test
   * Creates a new authority keypair and initializes the bankrun context
   */
  beforeEach(async () => {
    // Create fresh authority for each test
    authority = Keypair.generate();

    // Initialize bankrun context with funded authority account
    const context = await startAnchor(
      '.', // Current directory
      [], // No additional programs needed
      [
        {
          address: authority.publicKey,
          info: {
            lamports: 10 * LAMPORTS_PER_SOL, // Fund with 10 SOL
            data: Buffer.alloc(0),
            owner: anchor.web3.SystemProgram.programId,
            executable: false,
          },
        },
      ],
    );

    // Initialize provider and program with the new context
    provider = new BankrunProvider(context);
    program = new anchor.Program<Counter>(IDL, provider);
  });

  /**
   * Helper function to create a new counter account
   * @param authority - Keypair of the authority who will own the counter
   * @returns Object containing counter public key and initialization transaction signature
   */
  async function createCounter(authority: Keypair) {
    // Derive the counter PDA using authority's public key
    const [counter] = PublicKey.findProgramAddressSync([Buffer.from('counter'), authority.publicKey.toBuffer()], program.programId);

    // Initialize the counter account
    return {
      counter,
      tx: await program.methods
        .initialize()
        .accounts({
          authority: authority.publicKey,
          counter,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc(),
    };
  }

  /**
   * Test case: Verify counter initialization
   * Ensures a new counter starts with a count of zero
   */
  it('can initialize counter', async () => {
    const { counter } = await createCounter(authority);
    const counterAccount = await program.account.counterAccount.fetch(counter);
    assert.ok(counterAccount.count.eq(new anchor.BN(0)));
  });

  /**
   * Test case: Verify counter increment
   * Ensures the counter can be incremented by one
   */
  it('can increment counter', async () => {
    const { counter } = await createCounter(authority);

    // Perform increment operation
    await program.methods
      .increment()
      .accounts({
        authority: authority.publicKey,
        counter,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    // Verify counter was incremented
    const counterAccount = await program.account.counterAccount.fetch(counter);
    assert.ok(counterAccount.count.eq(new anchor.BN(1)));
  });

  /**
   * Test case: Verify counter separation
   * Ensures different authorities maintain separate counter states
   */
  it('maintains separate counts for different authorities', async () => {
    // Create and fund second authority
    const authority2 = Keypair.generate();
    const context = await startAnchor(
      '.',
      [],
      [
        // Fund first authority
        {
          address: authority.publicKey,
          info: {
            lamports: 10 * LAMPORTS_PER_SOL,
            data: Buffer.alloc(0),
            owner: anchor.web3.SystemProgram.programId,
            executable: false,
          },
        },
        // Fund second authority
        {
          address: authority2.publicKey,
          info: {
            lamports: 10 * LAMPORTS_PER_SOL,
            data: Buffer.alloc(0),
            owner: anchor.web3.SystemProgram.programId,
            executable: false,
          },
        },
      ],
    );

    // Reinitialize provider and program with new context
    provider = new BankrunProvider(context);
    program = new anchor.Program<Counter>(IDL, provider);

    // Create counters for both authorities
    const { counter: counter1 } = await createCounter(authority);
    const { counter: counter2 } = await createCounter(authority2);

    // Increment only the first counter
    await program.methods
      .increment()
      .accounts({
        authority: authority.publicKey,
        counter: counter1,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    // Fetch and verify both counter states
    const account1 = await program.account.counterAccount.fetch(counter1);
    const account2 = await program.account.counterAccount.fetch(counter2);

    // Verify first counter was incremented and second remains at zero
    assert.ok(account1.count.eq(new anchor.BN(1)));
    assert.ok(account2.count.eq(new anchor.BN(0)));
  });
});
