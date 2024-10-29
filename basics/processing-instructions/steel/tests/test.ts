import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

describe('custom-instruction-data', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'processing_instructions_steel_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  class InstructionData {
    name: Uint8Array;
    height: number;

    constructor(data: { name: Uint8Array; height: number }) {
      this.name = data.name;
      this.height = data.height;
    }

    static from(data: { name: string; height: number }) {
      return new InstructionData({
        name: Uint8Array.from(Buffer.from(data.name.padEnd(32, '\0'))), // 32 bytes
        height: data.height,
      });
    }

    static fromAccountData(accountData: Buffer) {
      const _accountData = Uint8Array.from(accountData).slice(8); // remove 8 byte discriminator

      return borsh.deserialize(InstructionDataSchema, InstructionData, Buffer.from(_accountData));
    }

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
          ['name', [32]], // 32 bytes
          ['height', 'u32'],
        ],
      },
    ],
  ]);

  test('Go to the park!', async () => {
    const blockhash = context.lastBlockhash;

    const jimmy = InstructionData.from({
      name: 'Jimmy',
      height: 3,
    });

    const mary = InstructionData.from({
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
