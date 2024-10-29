import assert from 'node:assert';
import { before, describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { BanksClient, BanksTransactionResultWithMeta, startAnchor } from 'solana-bankrun';
import type { CounterProgram } from '../target/types/counter_program';

const IDL = require('../target/idl/counter_program.json');

const PROGRAM_ID = new PublicKey(IDL.address);

async function createAndProcessTransaction(
  client: BanksClient,
  payer: Keypair,
  instruction: TransactionInstruction,
  additionalSigners: Keypair[] = [],
): Promise<BanksTransactionResultWithMeta> {
  const tx = new Transaction();
  // Get the latest blockhash
  const [latestBlockhash] = await client.getLatestBlockhash();
  tx.recentBlockhash = latestBlockhash;
  // Add transaction instructions
  tx.add(instruction);
  tx.feePayer = payer.publicKey;
  //Add signers
  tx.sign(payer, ...additionalSigners);
  // Process transaction
  const result = await client.tryProcessTransaction(tx);
  return result;
}

describe('counter_program', async () => {
  // Configure the client to use the anchor-bankrun
  const context = await startAnchor('', [{ name: 'counter_program', programId: PROGRAM_ID }], []);

  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;

  const program = new anchor.Program<CounterProgram>(IDL, provider);

  const counterKeypair = Keypair.generate(); // Generate a new user keypair

  before(async () => {
    //Transfer SOL to the user account to cover rent
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: counterKeypair.publicKey,
      lamports: 2 * LAMPORTS_PER_SOL,
    });

    await createAndProcessTransaction(context.banksClient, payer.payer, transferInstruction, [payer.payer]);
    const userBalance = await context.banksClient.getBalance(counterKeypair.publicKey);
    console.log(`User balance after funding: ${userBalance}`);
  });

  const [counter, _] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('count'), counterKeypair.publicKey.toBuffer()], program.programId);

  it('Initialize Counter', async () => {
    await program.methods
      .initializeCounter()
      .accounts({
        payer: counterKeypair.publicKey,
      })
      .signers([counterKeypair])
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
    await program.methods.decrement().accounts({ counter: counter }).rpc();
    await program.methods.decrement().accounts({ counter: counter }).rpc();
    const currentCount = await program.account.counter.fetch(counter);
    assert.strictEqual(currentCount.count.toNumber(), 0, 'Expected count to be 0 after multiple decrements');
  });
});
