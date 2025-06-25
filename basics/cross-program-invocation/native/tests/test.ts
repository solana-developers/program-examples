import { Buffer } from 'node:buffer';
import { readFileSync } from 'node:fs';
import { homedir } from 'node:os';
import { Connection, Keypair, SystemProgram, Transaction, TransactionInstruction, sendAndConfirmTransaction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';


function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(Buffer.from(JSON.parse(readFileSync(path, 'utf-8'))));
}

describe('CPI Example', async () => {
  //const connection = new Connection('http://localhost:8899', 'confirmed');

  const hand = createKeypairFromFile('./target/deploy/cross_program_invocatio_native_hand-keypair.json');
  const lever = createKeypairFromFile('./target/deploy/cross_program_invocatio_native_lever-keypair.json');


  const context = await start([
    { name: 'cross_program_invocatio_native_hand', programId: hand.publicKey },
    { name: 'cross_program_invocatio_native_lever', programId: lever.publicKey }
  ], [])

  const client = context.banksClient;
  const payer = context.payer;

  class Assignable {
    constructor(properties: any) {
      for (const [key, value] of Object.entries(properties)) {
        (this as any)[key] = value;
      }
    }
  }

  class PowerStatus extends Assignable {
    is_on!: number;

    toBuffer() {
      return Buffer.from(borsh.serialize(PowerStatusSchema, this));
    }
  }
  const PowerStatusSchema = new Map([[PowerStatus, { kind: 'struct', fields: [['is_on', 'u8']] }]]);

  class SetPowerStatus extends Assignable {
    name!: string;

    toBuffer() {
      return Buffer.from(borsh.serialize(SetPowerStatusSchema, this));
    }
  }
  const SetPowerStatusSchema = new Map([[SetPowerStatus, { kind: 'struct', fields: [['name', 'string']] }]]);

  const powerAccount = Keypair.generate();

  it('Initialize the lever!', async () => {
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: powerAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: lever.publicKey,
      data: new PowerStatus({ is_on: true }).toBuffer(),
    });


    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });

  it('Pull the lever!', async () => {
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: powerAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: lever.publicKey, isSigner: false, isWritable: false },
      ],
      programId: hand.publicKey,
      data: new SetPowerStatus({ name: 'Chris' }).toBuffer(),
    });

    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });

  it('Pull it again!', async () => {
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: powerAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: lever.publicKey, isSigner: false, isWritable: false },
      ],
      programId: hand.publicKey,
      data: new SetPowerStatus({ name: 'Ashley' }).toBuffer(),
    });


    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });
});
