import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import BN from 'bn.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';

const COUNTER_ACCOUNT_SIZE = 8 + 8; // 8 byte u64 count + 8 byte disciminator

type Counter = {
  count: BN;
};

function deserializeCounterAccount(data: Buffer): Counter {
  if (data.byteLength !== 16) {
    throw Error('Need exactly 16 bytes to deserialize counter');
  }

  return {
    count: new BN(data.slice(8), 'le'), // slice off the 8 byte descriminator to get the count
  };
}

describe('Counter Solana Native', async () => {
  // Randomly generate the program keypair and load the program to solana-bankrun
  const PROGRAM_ID = PublicKey.unique();

  const context = await start([{ name: 'counter_solana_steel', programId: PROGRAM_ID }], []);

  const client = context.banksClient;
  // Get the payer keypair from the context, this will be used to sign transactions with enough lamports
  const payer = context.payer;
  // Get the rent object to calculate rent for the accounts
  const rent = await client.getRent();

  function createIncrementInstruction(counter: PublicKey): TransactionInstruction {
    return new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        {
          pubkey: counter,
          isSigner: false,
          isWritable: true,
        },
      ],
      data: Buffer.from([0x0]),
    });
  }

  const counterKeypair = Keypair.generate();
  const counter = counterKeypair.publicKey;

  test('Test allocate counter + increment tx', async () => {
    // Randomly generate the account key
    // to sign for setting up the Counter state

    // Create a TransactionInstruction to interact with our counter program
    const allocIx = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: counter,
      lamports: Number(rent.minimumBalance(BigInt(COUNTER_ACCOUNT_SIZE))),
      space: COUNTER_ACCOUNT_SIZE,
      programId: PROGRAM_ID,
    });

    const incrementIx = createIncrementInstruction(counter);

    // create the counter, and then increment the count
    const tx = new Transaction().add(allocIx).add(incrementIx);

    // Explicitly set the feePayer to be our wallet (this is set to first signer by default)
    tx.feePayer = payer.publicKey;

    // Fetch a "timestamp" so validators know this is a recent transaction
    const blockhash = context.lastBlockhash;
    tx.recentBlockhash = blockhash;

    // Sign the transaction with the payer's keypair
    tx.sign(payer, counterKeypair);

    // Send transaction to bankrun
    await client.processTransaction(tx);

    // Get the counter account info from network
    const counterAccountInfo = await client.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    // Deserialize the counter & check count has been incremented
    const counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));

    assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');
    console.log(`[alloc+increment] count is: ${counterAccount.count.toNumber()}`);
  });

  test('Increment the counter again', async () => {
    let counterAccountInfo = await client.getAccount(counter);

    assert(counterAccountInfo, 'Expected counter account to have been created');

    let counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));

    assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');

    console.log(`[allocate] count is: ${counterAccount.count.toNumber()}`);

    // Check increment tx
    const incrementIx: TransactionInstruction = createIncrementInstruction(counter);

    const tx = new Transaction().add(incrementIx);
    tx.feePayer = payer.publicKey;

    const blockhash = context.lastBlockhash;
    tx.recentBlockhash = blockhash;
    tx.sign(payer);

    await client.processTransaction(tx);

    counterAccountInfo = await client.getAccount(counter);

    assert(counterAccountInfo, 'Expected counter account to have been created');

    counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));

    assert(counterAccount.count.toNumber() === 2, 'Expected count to have been 2');

    console.log(`[increment] count is: ${counterAccount.count.toNumber()}`);
  });
});
