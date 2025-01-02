import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { expect } from 'chai';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

// Constants for program identification
const PROGRAM_ID = new PublicKey('Dw6Yq7TZSHdaqB2nKjsxuDrdp5xYCuZaVKFZb5vp5Y4Y');
const ADDRESS_INFO_SEED = 'address_info';

// Instruction discriminators
const INSTRUCTION_DISCRIMINATORS = {
  createAddressInfo: Buffer.from([0]),
};

// Type definitions
interface AddressInfoData {
  name: string;
  houseNumber: bigint;
  street: string;
  city: string;
}

interface AddressInfoDataRaw {
  name: Uint8Array;
  houseNumber: bigint;
  street: Uint8Array;
  city: Uint8Array;
}

interface AddressInfoAccount {
  data: AddressInfoData;
}

interface AddressInfoAccountRaw {
  data: AddressInfoDataRaw;
}

// Borsh schema definition
const ADDRESS_INFO_SCHEMA: borsh.Schema = {
  struct: {
    discriminator: 'u64',
    data: {
      struct: {
        name: { array: { type: 'u8', len: 64 } },
        houseNumber: 'u64',
        street: { array: { type: 'u8', len: 64 } },
        city: { array: { type: 'u8', len: 64 } },
      },
    },
  },
};

// Helper functions
const stringToFixedBytes = (str: string, length: number): Uint8Array => {
  const buffer = Buffer.alloc(length, 0);
  buffer.write(str, 'utf-8');
  return Uint8Array.from(buffer);
};

const fixedBytesToString = (data: Uint8Array): string => {
  return Buffer.from(data).toString('utf-8').replace(/\0/g, '');
};

const bigIntToBytes = (value: bigint): Uint8Array => {
  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(value, 0);
  return Uint8Array.from(buffer);
};

const serializeAddressInfo = (data: AddressInfoData): Buffer => {
  return Buffer.concat([
    stringToFixedBytes(data.name, 64),
    bigIntToBytes(data.houseNumber),
    stringToFixedBytes(data.street, 64),
    stringToFixedBytes(data.city, 64),
  ]);
};

const deserializeAddressInfo = (raw: AddressInfoAccountRaw): AddressInfoAccount => {
  return {
    data: {
      name: fixedBytesToString(raw.data.name),
      houseNumber: raw.data.houseNumber,
      street: fixedBytesToString(raw.data.street),
      city: fixedBytesToString(raw.data.city),
    },
  };
};

describe('Address Info Program', () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  let addressInfoPda: PublicKey;

  before(async () => {
    // Initialize program test environment
    context = await start([{ name: 'account_data_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;

    // Derive program PDA
    [addressInfoPda] = PublicKey.findProgramAddressSync([Buffer.from(ADDRESS_INFO_SEED)], PROGRAM_ID);
  });

  describe('create_address_info', () => {
    it('successfully creates and initializes an address info account', async () => {
      // Test data
      const addressInfo = {
        name: 'Joe C',
        houseNumber: BigInt(136),
        street: 'Mile High Dr.',
        city: 'Solana Beach',
      };

      // Create and send transaction
      const tx = new Transaction().add(
        new TransactionInstruction({
          programId: PROGRAM_ID,
          keys: [
            { pubkey: payer.publicKey, isSigner: true, isWritable: true },
            { pubkey: addressInfoPda, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
          ],
          data: Buffer.concat([INSTRUCTION_DISCRIMINATORS.createAddressInfo, serializeAddressInfo(addressInfo)]),
        }),
      );

      tx.recentBlockhash = context.lastBlockhash;
      tx.sign(payer);

      await client.processTransaction(tx);

      // Verify account data
      const account = await client.getAccount(addressInfoPda);
      expect(account).to.not.be.null;

      const accountData = borsh.deserialize(ADDRESS_INFO_SCHEMA, account?.data) as AddressInfoAccountRaw;

      const deserializedData = deserializeAddressInfo(accountData);

      // Verify each field
      expect(deserializedData.data.name).to.equal(addressInfo.name);
      expect(deserializedData.data.houseNumber).to.equal(addressInfo.houseNumber);
      expect(deserializedData.data.street).to.equal(addressInfo.street);
      expect(deserializedData.data.city).to.equal(addressInfo.city);
    });
  });
});
