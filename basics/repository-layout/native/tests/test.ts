import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

describe('Carnival', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'repository_layout_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  class Assignable {
    constructor(properties) {
      for (const [key, value] of Object.entries(properties)) {
        this[key] = value;
      }
    }
  }

  class CarnivalInstruction extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(CarnivalInstructionSchema, this));
    }
  }

  const CarnivalInstructionSchema = new Map([
    [
      CarnivalInstruction,
      {
        kind: 'struct',
        fields: [
          ['name', 'string'],
          ['height', 'u32'],
          ['ticket_count', 'u32'],
          ['attraction', 'string'],
          ['attraction_name', 'string'],
        ],
      },
    ],
  ]);

  async function sendCarnivalInstructions(instructionsList: CarnivalInstruction[]) {
    const tx = new Transaction();
    for (const ix of instructionsList) {
      tx.recentBlockhash = context.lastBlockhash;
      tx.add(
        new TransactionInstruction({
          keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
          programId: PROGRAM_ID,
          data: ix.toBuffer(),
        }),
      ).sign(payer);
    }
    await client.processTransaction(tx);
  }

  test('Go on some rides!', async () => {
    await sendCarnivalInstructions([
      new CarnivalInstruction({
        name: 'Jimmy',
        height: 36,
        ticket_count: 15,
        attraction: 'ride',
        attraction_name: 'Scrambler',
      }),
      new CarnivalInstruction({
        name: 'Mary',
        height: 52,
        ticket_count: 1,
        attraction: 'ride',
        attraction_name: 'Ferris Wheel',
      }),
      new CarnivalInstruction({
        name: 'Alice',
        height: 56,
        ticket_count: 15,
        attraction: 'ride',
        attraction_name: 'Scrambler',
      }),
      new CarnivalInstruction({
        name: 'Bob',
        height: 49,
        ticket_count: 6,
        attraction: 'ride',
        attraction_name: 'Tilt-a-Whirl',
      }),
    ]);
  });

  test('Play some games!', async () => {
    await sendCarnivalInstructions([
      new CarnivalInstruction({
        name: 'Jimmy',
        height: 36,
        ticket_count: 15,
        attraction: 'game',
        attraction_name: 'I Got It!',
      }),
      new CarnivalInstruction({
        name: 'Mary',
        height: 52,
        ticket_count: 1,
        attraction: 'game',
        attraction_name: 'Ring Toss',
      }),
      new CarnivalInstruction({
        name: 'Alice',
        height: 56,
        ticket_count: 15,
        attraction: 'game',
        attraction_name: 'Ladder Climb',
      }),
      new CarnivalInstruction({
        name: 'Bob',
        height: 49,
        ticket_count: 6,
        attraction: 'game',
        attraction_name: 'Ring Toss',
      }),
    ]);
  });

  test('Eat some food!', async () => {
    await sendCarnivalInstructions([
      new CarnivalInstruction({
        name: 'Jimmy',
        height: 36,
        ticket_count: 15,
        attraction: 'food',
        attraction_name: 'Taco Shack',
      }),
      new CarnivalInstruction({
        name: 'Mary',
        height: 52,
        ticket_count: 1,
        attraction: 'food',
        attraction_name: "Larry's Pizza",
      }),
      new CarnivalInstruction({
        name: 'Alice',
        height: 56,
        ticket_count: 15,
        attraction: 'food',
        attraction_name: "Dough Boy's",
      }),
      new CarnivalInstruction({
        name: 'Bob',
        height: 49,
        ticket_count: 6,
        attraction: 'food',
        attraction_name: "Dough Boy's",
      }),
    ]);
  });
});
