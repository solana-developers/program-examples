import { Buffer } from 'node:buffer';
import { Connection, Keypair, SystemProgram, sendAndConfirmTransaction, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';

function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(require('node:fs').readFileSync(path, 'utf-8'))));
}

describe('CPI Example', () => {
  const connection = new Connection('http://localhost:8899', 'confirmed');
  const payer = createKeypairFromFile(`${require('node:os').homedir()}/.config/solana/id.json`);
  const hand = createKeypairFromFile('./target/so/hand-keypair.json');
  const lever = createKeypairFromFile('./target/so/lever-keypair.json');

  const PowerStatusSchema = { struct: { is_on: 'u8' } };
  const SetPowerStatusSchema = { struct: { name: 'string' } };

  function borshSerialize(schema: borsh.Schema, data: object): Buffer {
    return Buffer.from(borsh.serialize(schema, data));
  }

  const powerAccount = Keypair.generate();

  it('Initialize the lever!', async () => {
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: powerAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: lever.publicKey,
      data: borshSerialize(PowerStatusSchema, { is_on: true }),
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
      data: borshSerialize(SetPowerStatusSchema, { name: 'Chris' }),
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
      data: borshSerialize(SetPowerStatusSchema, { name: 'Ashley' }),
    });

    await sendAndConfirmTransaction(connection, new Transaction().add(ix), [payer]);
  });
});
