import { describe, test } from 'node:test';
import { PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';

describe('hello-solana', async () => {
  // load program in solana-bankrun
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start([{ name: 'steel_hello_solana', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  test('Say hello!', async () => {
    const blockhash = context.lastBlockhash;

    // pass in our name as the instruction data.
    const name = 'The Wuh';

    // We set up our instruction first.
    const ix = new TransactionInstruction({
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }], // Accounts we are passing in
      programId: PROGRAM_ID, // Our Program ID
      data: Buffer.from(name), // takes in a buffer
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    // Now we process the transaction
    const transaction = await client.processTransaction(tx);

    const logCount = transaction.logMessages.length;

    assert(transaction.logMessages[0].startsWith(`Program ${PROGRAM_ID}`));
    assert(transaction.logMessages[1] === `Program log: Hello, ${name}!`);
    assert(transaction.logMessages[2] === `Program log: Our program's Program ID: ${PROGRAM_ID}`);
    assert(transaction.logMessages[3] === `Program log: We have ${ix.keys.length} accounts`);
    assert(transaction.logMessages[logCount - 2].startsWith(`Program ${PROGRAM_ID} consumed`));
    assert(transaction.logMessages[logCount - 1] === `Program ${PROGRAM_ID} success`);
  });
});
