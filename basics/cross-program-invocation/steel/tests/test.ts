import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

describe('CPI Example', async () => {
  const LEVER_PROGRAM_ID = new PublicKey('E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN');
  const HAND_PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const powerAccount = Keypair.generate();

  const context = await start(
    [
      {
        name: 'cross_program_invocation_steel_lever',
        programId: LEVER_PROGRAM_ID,
      },
      {
        name: 'cross_program_invocation_steel_hand',
        programId: HAND_PROGRAM_ID,
      },
    ],
    [],
  );

  const client = context.banksClient;
  const payer = context.payer;

  class Assignable {
    constructor(properties) {
      for (const [key, value] of Object.entries(properties)) {
        this[key] = value;
      }
    }
  }

  class PowerStatus extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(PowerStatusSchema, this));
    }
  }
  const PowerStatusSchema = new Map([[PowerStatus, { kind: 'struct', fields: [['on', 'u8']] }]]);

  class SetPowerStatus {
    name: Uint8Array;

    constructor(name: Uint8Array) {
      this.name = name;
    }

    static from(props: { name: string }) {
      return new SetPowerStatus(Uint8Array.from(Buffer.from(props.name.padEnd(48, '\0'))));
    }

    toBuffer() {
      return Buffer.from(borsh.serialize(SetPowerStatusSchema, this));
    }
  }

  const SetPowerStatusSchema = new Map([[SetPowerStatus, { kind: 'struct', fields: [['name', [48]]] }]]);

  test('Initialize the lever!', async () => {
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: powerAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: LEVER_PROGRAM_ID,
      data: Buffer.concat([
        Buffer.from([0]), // the instruction discriminator
        new PowerStatus({ on: true }).toBuffer(),
      ]),
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, powerAccount);

    await client.processTransaction(tx);
  });

  test('Pull the lever!', async () => {
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: powerAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: LEVER_PROGRAM_ID, isSigner: false, isWritable: false },
      ],
      programId: HAND_PROGRAM_ID,
      data: SetPowerStatus.from({ name: 'Chris' }).toBuffer(),
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });

  test('Pull it again!', async () => {
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: powerAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: LEVER_PROGRAM_ID, isSigner: false, isWritable: false },
      ],
      programId: HAND_PROGRAM_ID,
      data: SetPowerStatus.from({ name: 'Ashley' }).toBuffer(),
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });
});
