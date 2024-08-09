import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { CounterAnchor } from '../target/types/counter_anchor';

const IDL = require('../target/idl/counter_anchor.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('counter_anchor', async () => {
  // Configure the client to use the anchor-bankrun
  const context = await startAnchor('', [{ name: 'counter_anchor', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CounterAnchor>(IDL, provider);

  // Generate a new keypair for the counter account
  const counterKeypair = new Keypair();

  it('Initialize Counter', async () => {
    await program.methods
      .initializeCounter()
      .accounts({
        counter: counterKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([counterKeypair])
      .rpc();

    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);

    assert(currentCount.count.toNumber() === 0, 'Expected initialized count to be 0');
  });

  it('Increment Counter', async () => {
    await program.methods.increment().accounts({ counter: counterKeypair.publicKey }).rpc();

    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);

    assert(currentCount.count.toNumber() === 1, 'Expected  count to be 1');
  });
});
