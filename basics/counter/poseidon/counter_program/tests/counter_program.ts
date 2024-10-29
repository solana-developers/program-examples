import { before, describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { assert } from 'chai';
import { CounterProgram } from '../target/types/counter_program';

describe('counter_program', () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;

  const [counter, _] = anchor.web3.PublicKey.findProgramAddressSync([anchor.utils.bytes.utf8.encode('count')], program.programId);

  it('Initialize Counter', async () => {
    await program.methods
      .initializeCounter()
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();

    const currentCount = await program.account.counter.fetch(counter);

    assert(currentCount.count.toNumber() === 0, 'Expected initialized count to be 0');
  });

  it('Increment Counter', async () => {
    await program.methods
      .increment()
      .accounts({
        counter: counter,
      })
      .rpc();

    const currentCount = await program.account.counter.fetch(counter);

    assert(currentCount.count.toNumber() === 1, 'Expected  count to be 1');
  });

  it('Increment Counter Again', async () => {
    await program.methods
      .increment()
      .accounts({
        counter: counter,
      })
      .rpc();

    const currentCount = await program.account.counter.fetch(counter);

    assert(currentCount.count.toNumber() === 2, 'Expected  count to be 2');
  });
  it('Decrement counter', async () => {
    await program.methods
      .decrement()
      .accounts({
        counter: counter,
      })
      .rpc();

    const currentCount = await program.account.counter.fetch(counter);
    assert(currentCount.count.toNumber() === 1, 'Expected  count to be 1');
  });
  it('Increment and decrement multiple times', async () => {
    // Increment the counter 5 times
    for (let i = 0; i < 5; i++) {
      await program.methods
        .increment()
        .accounts({
          counter: counter,
        })
        .rpc();
    }

    let currentCount = await program.account.counter.fetch(counter);
    assert.strictEqual(currentCount.count.toNumber(), 6, 'Expected count to be 6 after 5 increments');

    // Decrement the counter 4 times
    for (let i = 0; i < 4; i++) {
      await program.methods
        .decrement()
        .accounts({
          counter: counter,
        })
        .rpc();
    }

    currentCount = await program.account.counter.fetch(counter);
    assert.strictEqual(currentCount.count.toNumber(), 2, 'Expected count to be 2 after 4 decrements');
  });

  it('Cannot decrement below 0', async () => {
    // Decrement the counter to 0
    await program.methods
      .decrement()
      .accounts({
        counter: counter,
      })
      .rpc();

    await program.methods
      .decrement()
      .accounts({
        counter: counter,
      })
      .rpc();

    const currentCount = await program.account.counter.fetch(counter);

    assert.strictEqual(currentCount.count.toNumber(), 0, 'Expected count to be 0 after multiple decrements');
  });
});
