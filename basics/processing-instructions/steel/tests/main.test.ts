import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

type GoToTheParkData = {
  name: string;
  height: bigint;
};

type GoToTheParkDataRaw = {
  name: Uint8Array;
  height: bigint;
};

const encodeString = (str: string, length: number): Uint8Array => {
  const buffer = Buffer.alloc(length, 0);
  buffer.write(str, 'utf-8');
  return Uint8Array.from(buffer);
};

const decodeString = (data: Uint8Array): string => {
  return Buffer.from(data).toString('utf-8').replace(/\0/g, '');
};

const encodeBigInt = (value: bigint): Uint8Array => {
  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(value, 0);
  return Uint8Array.from(buffer);
};

const createGoToTheParkBuffer = (data: GoToTheParkData): Buffer => {
  const name = encodeString(data.name, 64);
  const height = encodeBigInt(data.height); // 8 bytes
  return Buffer.concat([name, height]);
};

const toGoToTheParkData = (data: GoToTheParkDataRaw): GoToTheParkData => {
  return {
    name: decodeString(data.name),
    height: data.height,
  };
};

describe('processing instructions', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;

  before(async () => {
    context = await start([{ name: 'processing_instructions_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('should go to the park!', async () => {
    // data for the instruction
    const jimmy: GoToTheParkData = {
      name: 'Jimmy',
      height: BigInt(3),
    };

    const mary: GoToTheParkData = {
      name: 'Mary',
      height: BigInt(10),
    };

    const jimmyBuffer = createGoToTheParkBuffer(jimmy);
    const maryBuffer = createGoToTheParkBuffer(mary);

    // create the instructions
    const ix1 = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
      data: jimmyBuffer,
    });

    const ix2 = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
      data: maryBuffer,
    });

    // send the transaction
    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix1).add(ix2).sign(payer);

    // process the transaction
    const result = await client.processTransaction(tx);
    assert.ok(result);

    // check the logs
    // we got 2 instructions, we must see 2 consecutive logs
    // - Welcome to the park, {name}!
    // - You are NOT tall enough... and You are tall enough...

    assert(!!result.logMessages.find((msg) => msg.includes(`Welcome to the park, ${jimmy.name}!`)));

    assert(!!result.logMessages.find((msg) => msg.includes('You are NOT tall enough')));

    assert(!!result.logMessages.find((msg) => msg.includes(`Welcome to the park, ${mary.name}!`)));

    assert(!!result.logMessages.find((msg) => msg.includes('You are tall enough')));
  });
});
