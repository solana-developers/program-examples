import { readFileSync } from 'node:fs';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, type TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { LiteSVM } from 'litesvm';
import { COUNTER_ACCOUNT_SIZE, createIncrementInstruction, deserializeCounterAccount } from '../ts/index.ts';

describe('Counter Solana Native', () => {
  // Load the program keypair
  const programKeypairPath = new URL(
    './fixtures/counter_solana_native-keypair.json',
    // @ts-ignore
    import.meta.url,
  ).pathname;
  const programKeypairData = JSON.parse(readFileSync(programKeypairPath, 'utf-8'));
  const programKeypair = Keypair.fromSecretKey(new Uint8Array(programKeypairData));
  const PROGRAM_ID = programKeypair.publicKey;

  const litesvm = new LiteSVM();
  // Get the payer keypair from the context, this will be used to sign transactions with enough lamports
  const payer = Keypair.generate();

  // Load the program
  const programPath = new URL(
    './fixtures/counter_solana_native.so',
    // @ts-ignore
    import.meta.url,
  ).pathname;
  litesvm.addProgramFromFile(PROGRAM_ID, programPath);

  // Fund the payer account
  litesvm.airdrop(payer.publicKey, BigInt(100000000000));

  test('Test allocate counter + increment tx', () => {
    // Randomly generate the account key
    // to sign for setting up the Counter state
    const counterKeypair = Keypair.generate();
    const counter = counterKeypair.publicKey;

    // Create a TransactionInstruction to interact with our counter program
    const allocIx: TransactionInstruction = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: counter,
      lamports: Number(litesvm.minimumBalanceForRentExemption(BigInt(COUNTER_ACCOUNT_SIZE))),
      space: COUNTER_ACCOUNT_SIZE,
      programId: PROGRAM_ID,
    });
    const incrementIx: TransactionInstruction = createIncrementInstruction({ counter }, PROGRAM_ID);
    const tx = new Transaction().add(allocIx).add(incrementIx);

    // Explicitly set the feePayer to be our wallet (this is set to first signer by default)
    tx.feePayer = payer.publicKey;

    // Fetch a "timestamp" so validators know this is a recent transaction
    tx.recentBlockhash = litesvm.latestBlockhash();

    // Sign the transaction with the payer's keypair
    tx.sign(payer, counterKeypair);

    // Send transaction to litesvm
    litesvm.sendTransaction(tx);

    // Get the counter account info from network
    const counterAccountInfo = litesvm.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    // Deserialize the counter & check count has been incremented
    const counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
    assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');
    console.log(`[alloc+increment] count is: ${counterAccount.count.toNumber()}`);
  });

  test('Test allocate tx and increment tx', () => {
    const counterKeypair = Keypair.generate();
    const counter = counterKeypair.publicKey;

    // Check allocate tx
    const allocIx: TransactionInstruction = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: counter,
      lamports: Number(litesvm.minimumBalanceForRentExemption(BigInt(COUNTER_ACCOUNT_SIZE))),
      space: COUNTER_ACCOUNT_SIZE,
      programId: PROGRAM_ID,
    });
    let tx = new Transaction().add(allocIx);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(payer, counterKeypair);

    litesvm.sendTransaction(tx);

    let counterAccountInfo = litesvm.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    let counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
    assert(counterAccount.count.toNumber() === 0, 'Expected count to have been 0');
    console.log(`[allocate] count is: ${counterAccount.count.toNumber()}`);

    // Check increment tx
    const incrementIx: TransactionInstruction = createIncrementInstruction({ counter }, PROGRAM_ID);
    tx = new Transaction().add(incrementIx);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(payer);

    litesvm.sendTransaction(tx);

    counterAccountInfo = litesvm.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
    assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');
    console.log(`[increment] count is: ${counterAccount.count.toNumber()}`);
  });
});
