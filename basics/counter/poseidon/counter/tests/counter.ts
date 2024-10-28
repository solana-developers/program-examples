import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SystemProgram } from '@solana/web3.js';
import { expect } from 'chai';
import { CounterProgram } from '../target/types/counter_program';

describe('counter program', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<CounterProgram>;

  // Generate a unique counter account for each test run
  const counterKeypair = anchor.web3.Keypair.generate();

  it('Initialize counter', async () => {
    try {
      // Initialize the counter account
      await program.methods
        .initialize()
        .accounts({
          counter: counterKeypair.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([counterKeypair])
        .rpc();

      // Fetch the created account
      const counterAccount = await program.account.counter.fetch(counterKeypair.publicKey);

      // Verify the counter is initialized to 0
      expect(counterAccount.count.toNumber()).to.equal(0);
    } catch (error) {
      console.error('Initialization error:', error);
      throw error;
    }
  });

  it('Increment counter', async () => {
    try {
      // Increment the counter
      await program.methods
        .increment()
        .accounts({
          counter: counterKeypair.publicKey,
          user: provider.wallet.publicKey,
        })
        .rpc();

      // Fetch the updated counter account
      const counterAccount = await program.account.counter.fetch(counterKeypair.publicKey);

      // Verify the counter was incremented
      expect(counterAccount.count.toNumber()).to.equal(1);

      // Optional: Print the current count for debugging
      console.log('Current count:', counterAccount.count.toNumber());
    } catch (error) {
      console.error('Increment error:', error);
      throw error;
    }
  });

  // Optional: Add a test to verify multiple increments
  it('Multiple increments', async () => {
    try {
      // Increment twice more
      for (let i = 0; i < 2; i++) {
        await program.methods
          .increment()
          .accounts({
            counter: counterKeypair.publicKey,
            user: provider.wallet.publicKey,
          })
          .rpc();
      }

      const counterAccount = await program.account.counter.fetch(counterKeypair.publicKey);

      // Counter should now be 3 (1 from previous test + 2 new increments)
      expect(counterAccount.count.toNumber()).to.equal(3);
    } catch (error) {
      console.error('Multiple increment error:', error);
      throw error;
    }
  });
});
