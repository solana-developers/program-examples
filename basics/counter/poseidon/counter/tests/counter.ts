import { Idl, Program, utils } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { startAnchor } from 'solana-bankrun';
import { ProgramTestContext } from 'solana-bankrun';
import CounterIDL from '../target/idl/counter.json';
import { Counter } from '../target/types/counter';

describe('counter', () => {
  let context: ProgramTestContext;
  let provider: BankrunProvider;
  let counterProgram: Program<Counter>;
  let counterPDA: PublicKey;

  before(async () => {
    context = await startAnchor('../counter', [], []);
    provider = new BankrunProvider(context);
    counterProgram = new Program(CounterIDL as Idl, provider) as unknown as Program<Counter>;

    [counterPDA] = PublicKey.findProgramAddressSync([utils.bytes.utf8.encode('count')], counterProgram.programId);
  });

  it('initialize', async () => {
    await counterProgram.methods
      .initialize()
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .rpc();

    const counter = await counterProgram.account.counterState.fetch(counterPDA);
    expect(counter.count.toNumber()).equal(0);
  });

  it('increment', async () => {
    await counterProgram.methods.increment().rpc();

    const counter = await counterProgram.account.counterState.fetch(counterPDA);
    expect(counter.count.toNumber()).equal(1);
  });

  it('decrement', async () => {
    await counterProgram.methods.decrement().rpc();

    const counter = await counterProgram.account.counterState.fetch(counterPDA);
    expect(counter.count.toNumber()).equal(0);
  });
});
