import assert from 'node:assert';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { CounterAnchor } from '../target/types/counter_anchor';
const IDL = require('../target/idl/counter_anchor.json');

describe('counter anchor program', () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<CounterAnchor>;
  let payer: Keypair;
  let counterKeypair: Keypair;

  before(async () => {
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new Program<CounterAnchor>(IDL, provider);

    // a keypair for the counter account
    counterKeypair = new Keypair();
  });

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

    assert.equal(currentCount.count.toNumber(), 0, 'Expected initialized count to be 0');
  });

  it('Increment Counter', async () => {
    await program.methods.increment().accounts({ counter: counterKeypair.publicKey }).rpc();

    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);

    assert.equal(currentCount.count.toNumber(), 1, 'Expected  count to be 1');
  });
});
