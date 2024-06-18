import { Buffer } from 'node:buffer';
import { Connection, Keypair, SystemProgram, Transaction, TransactionInstruction, sendAndConfirmTransaction } from '@solana/web3.js';
import * as borsh from 'borsh';

function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(Buffer.from(JSON.parse(require('node:fs').readFileSync(path, 'utf-8'))));
}

describe('CPI Example', () => {
  const connection = new Connection('http://localhost:8899', 'confirmed');
  const payer = createKeypairFromFile(`${require('node:os').homedir()}/.config/solana/id.json`);
  const hand = createKeypairFromFile('./target/so/hand-keypair.json');
  const lever = createKeypairFromFile('./target/so/lever-keypair.json');

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
  const PowerStatusSchema = new Map([[PowerStatus, { kind: 'struct', fields: [['is_on', 'u8']] }]]);

  class SetPowerStatus extends Assignable {
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

    await sendAndConfirmTransaction(connection, new Transaction().add(ix), [payer, powerAccount]);
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

    await sendAndConfirmTransaction(connection, new Transaction().add(ix), [payer]);
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

    await sendAndConfirmTransaction(connection, new Transaction().add(ix), [payer]);
  });
});
