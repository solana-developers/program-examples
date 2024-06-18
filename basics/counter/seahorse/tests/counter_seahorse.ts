import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import type { CounterSeahorse } from '../target/types/counter_seahorse';

describe('counter_seahorse', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CounterSeahorse as Program<CounterSeahorse>;

  it('Increment counter', async () => {
    const seed = 69;
    const counter = PublicKey.findProgramAddressSync([Buffer.from([0x45])], program.programId)[0];

    // Initialize counter
    await program.methods
      .initializeCounter(seed)
      .accounts({
        payer: program.provider.publicKey,
      })
      .rpc();

    // Increment counter
    await program.methods
      .increment()
      .accounts({
        counter,
      })
      .rpc();

    const count = (await program.account.counter.fetch(counter)).count.toNumber();
    assert(count === 1, 'Expected count to be 1');
  });
});
