import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { CounterProgram } from '../target/types/counter_program';

const IDL = require('../target/idl/counter_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('counter_program', async () => {
  const context = await startAnchor('', [{ name: 'counter_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CounterProgram>(IDL, provider);

  const [stateAccount, _] = anchor.web3.PublicKey.findProgramAddressSync([anchor.utils.bytes.utf8.encode('count')], program.programId);

  it('Initialize the counter', async () => {
    await program.methods
      .initialize()
      .accounts({
        state: stateAccount as PublicKey,
        user: payer.publicKey,
      })
      .signers([{ publicKey: payer.publicKey, secretKey: payer.payer.secretKey }])
      .rpc();

    const account = await program.account.counterState.fetch(stateAccount);
    console.log('Counter after initialization:', account.count.toString());
    // Expecting the count to be 0
    expect(account.count.toString()).to.equal('0');
  });

  it('Increment the counter', async () => {
    await program.methods
      .increment()
      .accounts({
        state: stateAccount,
      })
      .rpc();

    const account = await program.account.counterState.fetch(stateAccount);
    console.log('Counter after increment:', account.count.toString());
    expect(account.count.toString()).to.equal('1');
  });
});
