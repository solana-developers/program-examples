import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

class AddressInfo extends Assignable {
  street: any;
  city: any;
  name: any;
  house_number: any;
  toBuffer() {
    return Buffer.from(borsh.serialize(AddressInfoSchema, this));
  }

  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(AddressInfoSchema, AddressInfo, buffer);
  }
}
const AddressInfoSchema = new Map([
  [
    AddressInfo,
    {
      kind: 'struct',
      fields: [
        ['name', 'string'],
        ['house_number', 'u8'],
        ['street', 'string'],
        ['city', 'string'],
      ],
    },
  ],
]);

describe('Account Data!', async () => {
  const addressInfoAccount = Keypair.generate();
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'account_data_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;

  test('Create the address info account', async () => {
    const payer = context.payer;

    console.log(`Program Address      : ${PROGRAM_ID}`);
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: addressInfoAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: new AddressInfo({
        name: 'Joe C',
        house_number: 136,
        street: 'Mile High Dr.',
        city: 'Solana Beach',
      }).toBuffer(),
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, addressInfoAccount);
    await client.processTransaction(tx);
  });

  test("Read the new account's data", async () => {
    const accountInfo = await client.getAccount(addressInfoAccount.publicKey);

    const readAddressInfo = AddressInfo.fromBuffer(Buffer.from(accountInfo.data));
    console.log(`Name     : ${readAddressInfo.name}`);
    console.log(`House Num: ${readAddressInfo.house_number}`);
    console.log(`Street   : ${readAddressInfo.street}`);
    console.log(`City     : ${readAddressInfo.city}`);
  });
});
