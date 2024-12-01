import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

describe('counter program', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;

  before(async () => {
    context = await start([{ name: 'counter_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('initialize and increment the counter', async () => {
    // derive the counter PDA
    const [counterPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('counter')], // seed
      PROGRAM_ID,
    );

    const instructionDiscriminators = {
      initialize: Buffer.from([0]),
      increment: Buffer.from([1]),
    };

    // create the initialize instruction
    const initializeIx = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: counterPDA, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: instructionDiscriminators.initialize,
    });

    // send the initialize transaction
    const initializeTx = new Transaction();
    initializeTx.recentBlockhash = context.lastBlockhash;
    initializeTx.add(initializeIx).sign(payer);

    // process the transaction
    await client.processTransaction(initializeTx);

    // fetch the counter account data
    const accountInfo = await client.getAccount(counterPDA);
    assert(accountInfo !== null, 'counter account should exist');

    // define the counter schema
    const counterSchema: borsh.Schema = {
      struct: { discriminator: 'u64', value: 'u64' },
    };

    // deserialize the counter account data
    const counterData = borsh.deserialize(counterSchema, accountInfo?.data) as {
      value: bigint;
    };

    // check the counter value is 0
    assert(counterData.value === BigInt(0), 'counter value should be 0');

    // increment (must be a number between 0 and 255)
    const amount = BigInt(42);
    const amountBuffer = Buffer.alloc(8);
    amountBuffer.writeBigUInt64LE(amount);

    // data for the increment instruction
    const incrementData = Buffer.concat([instructionDiscriminators.increment, amountBuffer]);

    // create the increment instruction
    const incrementIx = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: counterPDA, isSigner: false, isWritable: true },
      ],
      data: incrementData,
    });

    // send the increment transaction
    const incrementTx = new Transaction();
    incrementTx.recentBlockhash = context.lastBlockhash;
    incrementTx.add(incrementIx).sign(payer);

    // process the transaction
    await client.processTransaction(incrementTx);

    // fetch the counter account data
    const updatedAccountInfo = await client.getAccount(counterPDA);
    assert(updatedAccountInfo !== null, 'counter account should exist');

    // deserialize the updated counter account data
    const updatedCounterData = borsh.deserialize(counterSchema, updatedAccountInfo?.data) as { value: bigint };

    assert(updatedCounterData.value === BigInt(amount), `counter value should be ${amount} but we got ${updatedCounterData.value}`);
  });
});
