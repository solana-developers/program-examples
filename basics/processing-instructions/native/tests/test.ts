import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

describe('custom-instruction-data', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'processing_instructions_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  class Assignable {
    constructor(properties) {
      for (const [key, value] of Object.entries(properties)) {
        this[key] = value;
      }
    }
  }

  class InstructionData extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(InstructionDataSchema, this));
    }
  }

  const InstructionDataSchema = new Map([
    [
      InstructionData,
      {
        kind: 'struct',
        fields: [
          ['name', 'string'],
          ['height', 'u32'],
        ],
      },
    ],
  ]);

  test('Go to the park!', async () => {
    const blockhash = context.lastBlockhash;

    const jimmy = new InstructionData({
      name: 'Jimmy',
      height: 3,
    });

    const mary = new InstructionData({
      name: 'Mary',
      height: 10,
    });

    const ix1 = new TransactionInstruction({
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
      programId: PROGRAM_ID,
      data: jimmy.toBuffer(),
    });

    const ix2 = new TransactionInstruction({
      ...ix1,
      data: mary.toBuffer(),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix1).add(ix2).sign(payer);

    await client.processTransaction(tx);
  });
});
