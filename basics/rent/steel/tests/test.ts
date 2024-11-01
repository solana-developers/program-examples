import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { start } from 'solana-bankrun';

describe('Pay Rent!', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'rent_steel_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  class AddressData {
    name: string;
    address: string;

    constructor(props: { name: string; address: string }) {
      this.name = props.name;
      this.address = props.address;
    }

    toBuffer() {
      return Buffer.from(borsh.serialize(AddressDataSchema, this));
    }
  }

  const AddressDataSchema = new Map([
    [
      AddressData,
      {
        kind: 'struct',
        fields: [
          ['name', 'string'],
          ['address', 'string'],
        ],
      },
    ],
  ]);

  test('Create the account, pay rent', async () => {
    const newKeypair = Keypair.generate();

    const addressData = new AddressData({
      name: 'Marcus',
      address: '123 Main St. San Francisco, CA',
    });

    // We're just going to serialize our object here so we can check
    // the size on the client side against the program logs
    const addressDataBuffer = addressData.toBuffer();
    console.log(`Address data buffer length: ${addressDataBuffer.length}`);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: newKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: addressDataBuffer,
    });

    const tx = new Transaction();
    const blockhash = context.lastBlockhash;
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, newKeypair);

    await client.processTransaction(tx);

    // get mininum balance of rent exempt
    const expectedLamport = (await client.getRent()).minimumBalance(BigInt(addressDataBuffer.length));

    // get lamport of the account
    const actualLamport = await client.getBalance(newKeypair.publicKey);

    assert.equal(actualLamport, expectedLamport);
  });
});
