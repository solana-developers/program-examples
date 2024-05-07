import {
  PublicKey,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js';
import { start } from 'solana-bankrun';
import { describe, test } from 'node:test';

describe('hello-solana', async () => {
  // load program in solana-bankrun
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'hello_solana_program', programId: PROGRAM_ID }],[]);
  const client = context.banksClient;
  const payer = context.payer;

  test('Say hello!', async () => {
    const blockhash = context.lastBlockhash;
    // We set up our instruction first.
    let ix = new TransactionInstruction({
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
      programId: PROGRAM_ID,
      data: Buffer.alloc(0), // No data
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    // Now we process the transaction
    await client.processTransaction(tx);
  });
});
