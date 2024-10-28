import { before, describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { ProgramTestContext, startAnchor } from 'solana-bankrun';
import type { ProcessingInstructions } from '../target/types/processing_instructions';

const IDL = require('../target/idl/processing_instructions.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('processing-instructions', async () => {
  let context: ProgramTestContext;
  let provider: BankrunProvider;
  let program: Program<ProcessingInstructions>;

  before(async () => {
    context = await startAnchor('', [{ name: 'processing_instructions', programId: PROGRAM_ID }], []);
    provider = new BankrunProvider(context);
    program = new Program<ProcessingInstructions>(IDL, provider as unknown as anchor.Provider);
  });

  it('Initializes a greeting', async () => {
    const [greeting] = PublicKey.findProgramAddressSync([Buffer.from('greeting')], PROGRAM_ID);

    const timestamp = new Date().getTime() / 1000;

    // await program.methods.initialize('Hello, Solana!', new anchor.BN(timestamp)).accounts({}).rpc();
    await program.methods.initialize(new anchor.BN(timestamp)).accounts({}).rpc();

    console.log('Greeting initialized at:', greeting.toBase58());

    const [greetingAddress] = PublicKey.findProgramAddressSync([Buffer.from('greeting')], PROGRAM_ID);
    const greetingAccount = await program.account.greetingAccount.fetch(greetingAddress);

    expect(greetingAccount.lastUpdated.toNumber()).equal(Math.floor(timestamp));
  });

  it('Updates the greeting', async () => {
    const [greeting] = PublicKey.findProgramAddressSync([Buffer.from('greeting')], PROGRAM_ID);

    const timestamp = new Date().getTime() / 1000;

    // await program.methods.updateGreeting('Hello, Blockchain!', new anchor.BN(timestamp)).accounts({}).rpc();
    await program.methods
      .updateGreeting(new anchor.BN(timestamp))
      .accounts({
        greeting: greeting,
      })
      .rpc();

    const [greetingAddress] = PublicKey.findProgramAddressSync([Buffer.from('greeting')], PROGRAM_ID);
    const greetingAccount = await program.account.greetingAccount.fetch(greetingAddress);

    expect(greetingAccount.lastUpdated.toNumber()).equal(Math.floor(timestamp));
  });
});
