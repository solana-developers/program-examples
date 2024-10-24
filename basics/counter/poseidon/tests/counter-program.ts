import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { assert } from 'chai';
import { CounterProgramPoseidon } from '../target/types/counter_program_poseidon';

describe('counter', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CounterProgramPoseidon as Program<CounterProgramPoseidon>;
  const payer = provider.wallet as anchor.Wallet;

  // Generate a new keypair for the counter account
  const counterKeypair = new Keypair();

  it('Initializes the Counter', async () => {
    // Initialize the counter state
    await program.methods
      .initializeCounter()
      .accounts({
        counter: counterKeypair.publicKey,
        user: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the current state of the counter
    const currentCount = await program.account.counterState.fetch(counterKeypair.publicKey);

    // Assert that the counter was initialized to 0
    assert(currentCount.count.toNumber() === 0, 'Expected initialized count to be 0');
  });

  it('Increments the Counter', async () => {
    // Call the increment method
    await program.methods
      .incrementCounter()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .rpc();

    // Fetch the updated counter state
    const currentCount = await program.account.counterState.fetch(counterKeypair.publicKey);

    //   // Assert that the counter was incremented to 1
    assert(currentCount.count.toNumber() === 1, 'Expected count to be 1');
  });

  it('Decrements the Counter', async () => {
    // Decrement the counter
    await program.methods
      .decrementCounter()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .rpc();

    // Fetch the updated counter state
    const currentCount = await program.account.counterState.fetch(counterKeypair.publicKey);

    // Assert that the counter was decremented to 1
    assert(currentCount.count.toNumber() === 1, 'Expected count to be 1 after decrement');
  });
});
