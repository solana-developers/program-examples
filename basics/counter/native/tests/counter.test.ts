import { describe, test } from 'node:test';
import { Keypair, SystemProgram, Transaction, type TransactionInstruction, LAMPORTS_PER_SOL} from '@solana/web3.js';
import { assert } from 'chai';
import { LiteSVM } from 'litesvm';
import { COUNTER_ACCOUNT_SIZE, createIncrementInstruction, deserializeCounterAccount, PROGRAM_ID } from '../ts';

describe('Counter Solana Native', async () => {
  // Randomly generate the program keypair and load the program to solana-bankrun
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/counter_solana_native.so');
  
  // Get the payer keypair from the context, this will be used to sign transactions with enough lamports
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));
  // Get the rent object to calculate rent for the accounts
  const rent = svm.getRent();

  test('Test allocate counter + increment tx', async () => {
    // Randomly generate the account key
    // to sign for setting up the Counter state
    const counterKeypair = Keypair.generate();
    const counter = counterKeypair.publicKey;

    // Create a TransactionInstruction to interact with our counter program
    const allocIx: TransactionInstruction = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: counter,
      lamports: Number(rent.minimumBalance(BigInt(COUNTER_ACCOUNT_SIZE))),
      space: COUNTER_ACCOUNT_SIZE,
      programId: PROGRAM_ID,
    });
    const incrementIx: TransactionInstruction = createIncrementInstruction({ counter });
    const tx = new Transaction().add(allocIx).add(incrementIx);

    // Explicitly set the feePayer to be our wallet (this is set to first signer by default)
    tx.feePayer = payer.publicKey;

    // Fetch a "timestamp" so validators know this is a recent transaction
    const blockhash = svm.latestBlockhash();
    tx.recentBlockhash = blockhash;

    // Sign the transaction with the payer's keypair
    tx.sign(payer, counterKeypair);

    // Send transaction to litesvm
    svm.sendTransaction(tx);

    // Get the counter account info from network
    const counterAccountInfo = svm.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    // Deserialize the counter & check count has been incremented
    const counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
    assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');
    console.log(`[alloc+increment] count is: ${counterAccount.count.toNumber()}`);
  });

  test('Test allocate tx and increment tx', async () => {
    const counterKeypair = Keypair.generate();
    const counter = counterKeypair.publicKey;

    // Check allocate tx
    const allocIx: TransactionInstruction = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: counter,
      lamports: Number(rent.minimumBalance(BigInt(COUNTER_ACCOUNT_SIZE))),
      space: COUNTER_ACCOUNT_SIZE,
      programId: PROGRAM_ID,
    });
    let tx = new Transaction().add(allocIx);
    const blockhash = svm.latestBlockhash();
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = blockhash;
    tx.sign(payer, counterKeypair);

    svm.sendTransaction(tx);

    let counterAccountInfo = svm.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    let counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
    assert(counterAccount.count.toNumber() === 0, 'Expected count to have been 0');
    console.log(`[allocate] count is: ${counterAccount.count.toNumber()}`);

    // Check increment tx
    const incrementIx: TransactionInstruction = createIncrementInstruction({ counter });
    tx = new Transaction().add(incrementIx);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = blockhash;
    tx.sign(payer);

    svm.sendTransaction(tx);

    counterAccountInfo = svm.getAccount(counter);
    assert(counterAccountInfo, 'Expected counter account to have been created');

    counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
    assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');
    console.log(`[increment] count is: ${counterAccount.count.toNumber()}`);
  });
});
