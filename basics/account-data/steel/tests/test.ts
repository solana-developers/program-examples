import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

class AddressInfo {
  name: Uint8Array;
  house_number: number;
  street: Uint8Array;
  city: Uint8Array;

  constructor(info: {
    name: Uint8Array;
    house_number: number;
    street: Uint8Array;
    city: Uint8Array;
  }) {
    this.name = info.name;
    this.house_number = info.house_number;
    this.street = info.street;
    this.city = info.city;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(AddressInfoSchema, this));
  }

  static fromAccountData(buffer: Buffer) {
    const _accountData = Uint8Array.from(buffer).slice(8); // remove 8 byte discriminator

    return borsh.deserialize(AddressInfoSchema, AddressInfo, Buffer.from(_accountData));
  }

  static fromBuffer(buffer: Buffer) {
    const some = Uint8Array.from(buffer).slice(8);

    return borsh.deserialize(AddressInfoSchema, AddressInfo, Buffer.from(some));
  }

  static fromData(info: {
    name: string;
    house_number: number;
    street: string;
    city: string;
  }) {
    return new AddressInfo({
      name: Uint8Array.from(Buffer.from(info.name.padEnd(48, '\0'))),
      city: Uint8Array.from(Buffer.from(info.city.padEnd(48, '\0'))),
      street: Uint8Array.from(Buffer.from(info.street.padEnd(48, '\0'))),
      house_number: info.house_number,
    });
  }

  toData() {
    return {
      name: Buffer.from(this.name).toString(),
      city: Buffer.from(this.city).toString(),
      house_number: this.house_number,
      street: Buffer.from(this.street).toString(),
    };
  }
}
const AddressInfoSchema = new Map([
  [
    AddressInfo,
    {
      kind: 'struct',
      fields: [
        ['name', [48]], // Fixed array of 48 bytes
        ['house_number', 'u8'],
        ['street', [48]], // Fixed array of 48 bytes
        ['city', [48]], // Fixed array of 48 bytes
      ],
    },
  ],
]);

describe('Account Data!', async () => {
  const addressInfoAccount = Keypair.generate();
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'account_data_steel_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;

  test('Create the address info account', async () => {
    const payer = context.payer;

    console.log(`Program Address    : ${PROGRAM_ID}`);
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        {
          pubkey: addressInfoAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: AddressInfo.fromData({
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

    const readAddressInfo = AddressInfo.fromAccountData(Buffer.from(accountInfo.data)).toData();

    console.log(`Name     : ${readAddressInfo.name}`);
    console.log(`House Num: ${readAddressInfo.house_number}`);
    console.log(`Street   : ${readAddressInfo.street}`);
    console.log(`City     : ${readAddressInfo.city}`);
  });
});
