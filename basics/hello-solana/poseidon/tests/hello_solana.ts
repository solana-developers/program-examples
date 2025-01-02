import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { startAnchor } from 'solana-bankrun';
import { HelloSolana } from '../target/types/hello_solana';

const IDL = require('../target/idl/hello_solana.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('hello-solana', async () => {
  // Configure the Anchor provider & load the program IDL for anchor-bankrun
  // The IDL gives you a typescript module
  const context = await startAnchor('', [{ name: 'hello_solana', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const payer = provider.wallet as anchor.Wallet;

  const program = new anchor.Program<HelloSolana>(IDL, provider);

  it('Say hello!', async () => {
    // Just run Anchor's IDL method to build a transaction!
    await program.methods.initialize().accounts({}).signers([payer.payer]).rpc();
  });

  it('Increment', async () => {
    const [counterAddress] = PublicKey.findProgramAddressSync([Buffer.from('counter'), payer.publicKey.toBuffer()], PROGRAM_ID);

    await program.methods.increment().accounts({ counter: counterAddress }).signers([payer.payer]).rpc();

    const counter = await program.account.counter.fetch(counterAddress);
    expect(counter.value).equal(1);
  });

  it('Increment again', async () => {
    const [counterAddress] = PublicKey.findProgramAddressSync([Buffer.from('counter'), payer.publicKey.toBuffer()], PROGRAM_ID);

    await program.methods.increment().accounts({ counter: counterAddress }).signers([payer.payer]).rpc();

    const counter = await program.account.counter.fetch(counterAddress);
    expect(counter.value).equal(2);
  });
});
