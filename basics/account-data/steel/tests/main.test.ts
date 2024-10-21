import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

const instructionDiscriminators = {
  createAddressInfo: Buffer.from([0]),
};

type AddressInfoAccount = {
  data: {
    name: string;
    house_number: bigint;
    street: string;
    city: string;
  };
};

type AddressInfoAccountRaw = {
  data: {
    name: Uint8Array;
    house_number: bigint;
    street: Uint8Array;
    city: Uint8Array;
  };
};

const addressInfoSchema: borsh.Schema = {
  struct: {
    discriminator: 'u64',
    data: {
      struct: {
        name: { array: { type: 'u8', len: 64 } },
        house_number: 'u64',
        street: { array: { type: 'u8', len: 64 } },
        city: { array: { type: 'u8', len: 64 } },
      },
    },
  },
};

const encodeString = (str: string, length: number): Uint8Array => {
  const buffer = Buffer.alloc(length, 0);
  buffer.write(str, 'utf-8');
  return Uint8Array.from(buffer);
};

const decodeString = (data: Uint8Array): string => {
  return Buffer.from(data).toString('utf-8').replace(/\0/g, '');
};

const encodeBigInt = (value: bigint): Uint8Array => {
  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(value, 0);
  return Uint8Array.from(buffer);
};

const createAddressInfo = (data: AddressInfoAccount['data']): Buffer => {
  const name = encodeString(data.name, 64);
  const house_number = encodeBigInt(data.house_number); // 8 bytes
  const street = encodeString(data.street, 64);
  const city = encodeString(data.city, 64);
  return Buffer.concat([name, house_number, street, city]);
};

const toAddressInfoAccount = (data: AddressInfoAccountRaw): AddressInfoAccount => {
  return {
    data: {
      name: decodeString(data.data.name),
      house_number: data.data.house_number,
      street: decodeString(data.data.street),
      city: decodeString(data.data.city),
    },
  };
};

describe('Account data program', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;

  before(async () => {
    context = await start([{ name: 'account_data_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('should create the address info account', async () => {
    // derive the PDA
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from('address_info')], // seed
      PROGRAM_ID,
    );

    // data for the instruction
    const addressInfoData: AddressInfoAccount['data'] = {
      name: 'Alice',
      house_number: BigInt(42),
      street: 'Wonderland',
      city: 'Solana Beach',
    };

    // discriminator 8 bytes + data (name 64 bytes + house_number 8 bytes + street 64 bytes + city 64 bytes) = 8 + 200 bytes
    const addressInfoDataBuffer = createAddressInfo(addressInfoData);

    // data for the instruction
    const data = Buffer.concat([instructionDiscriminators.createAddressInfo, addressInfoDataBuffer]);

    // create the instruction
    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: pda, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data,
    });

    // send the transaction
    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    // fetch the account data
    const accountInfo = await client.getAccount(pda);
    assert(accountInfo !== null, 'account should exist');

    // deserialize the account data
    const rawAccountData = borsh.deserialize(addressInfoSchema, accountInfo.data) as AddressInfoAccountRaw;

    assert.isNotNull(rawAccountData, 'account data should exist');

    const accountData = toAddressInfoAccount(rawAccountData);

    // check the data
    assert(accountData.data.name === addressInfoData.name, `name should be ${addressInfoData.name} but we got ${accountData.data.name}`);
    assert(
      accountData.data.house_number === addressInfoData.house_number,
      `house number should be ${addressInfoData.house_number} but we got ${accountData.data.house_number}`,
    );
    assert(accountData.data.street === addressInfoData.street, `street should be ${addressInfoData.street} but we got ${accountData.data.street}`);
    assert(accountData.data.city === addressInfoData.city, `city should be ${addressInfoData.city} but we got ${accountData.data.city}`);
  });
});
