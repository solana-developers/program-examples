import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import BN from 'bn.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';

type Counter = {
  count: BN;
};

function deserializeCounterAccount(data: Uint8Array): Counter {
  if (data.byteLength !== 16) {
    throw Error('Need exactly 16 bytes to deserialize counter');
  }

  return {
    count: new BN(data.slice(8), 'le'), // slice off the 8 byte descriminator to get the count
  };
}

enum CounterInstruction {
  Initialize = 0,
  Increment = 1,
}

describe('Counter Solana Steel!', async () => {
  // Load the program id
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

  const context = await start([{ name: 'counter_solana_steel', programId: PROGRAM_ID }], []);

  const client = context.banksClient;
  // Get the payer keypair from the context, this will be used to sign transactions with enough lamports
  const payer = context.payer;

  // our counter address derived from the seeds
  const counter = PublicKey.findProgramAddressSync([Buffer.from('counter')], PROGRAM_ID)[0];

  test('Initialize counter', async () => {
    // Let's create the initialize counter instruction
    const incrementIx = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        {
          pubkey: counter,
          isSigner: false,
          isWritable: true, // set to true so we can modify its data
        },
        {
          pubkey: payer.publicKey, // payer publickey
          isSigner: true, // make sure it is a signer
          isWritable: true, // make sure it is writable
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: true,
        },
      ],
      data: Buffer.from([CounterInstruction.Initialize]),
    });

    const tx = new Transaction().add(incrementIx);

    // Explicitly set the feePayer to be our wallet (this is set to first signer by default)
    tx.feePayer = payer.publicKey;

    // Fetch a "timestamp" so validators know this is a recent transaction
    const blockhash = context.lastBlockhash;
    tx.recentBlockhash = blockhash;

    // Sign the transaction with the payer's keypair
    tx.sign(payer);

    // Send transaction to bankrun
    await client.processTransaction(tx);

    // Get the counter account info from network
    const counterAccountInfo = await client.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    // Deserialize the counter & check count has been incremented
    const counterAccount = deserializeCounterAccount(counterAccountInfo.data);
    assert(counterAccount.count.toNumber() === 0, 'Expected count to have been 0');

    console.log(`[initialize] count is: ${counterAccount.count.toNumber()}`);
  });

  test('Increment counter!', async () => {
    // let's create the increment counter instruction
    const incrementIx: TransactionInstruction = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        {
          pubkey: counter,
          isSigner: false,
          isWritable: true,
        },
      ],
      data: Buffer.from([CounterInstruction.Increment]),
    });

    const tx = new Transaction().add(incrementIx);
    tx.feePayer = payer.publicKey;

    const blockhash = context.lastBlockhash;
    tx.recentBlockhash = blockhash;
    tx.sign(payer);

    await client.processTransaction(tx);

    const counterAccountInfo = await client.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    const counterAccount = deserializeCounterAccount(counterAccountInfo.data);
    assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');

    console.log(`[increment] count is: ${counterAccount.count.toNumber()}`);
  });
});
