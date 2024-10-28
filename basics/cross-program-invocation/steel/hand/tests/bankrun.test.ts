import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';

import { ProgramTestContext, BanksClient, start } from 'solana-bankrun';
import * as borsh from 'borsh';
import { describe, it } from 'mocha';
import { assert } from 'chai';
const LEVER_PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
const PROGRAM_ID = new PublicKey('2LSt6uKm3YpTogXwEUvSNWLskMdsA6uyNkBFhwkS7sx4');

export const encodeString = (str: string, length: number): Uint8Array => {
  const buffer = Buffer.alloc(length, 0);
  buffer.write(str, 'utf-8');
  return Uint8Array.from(buffer);
};

export const decodeString = (data: Uint8Array): string => {
  return Buffer.from(data).toString('utf-8').replace(/\0/g, '');
};
const instructionDiscriminators = {
  Initialize: Buffer.from([0]),
  SwitchPower: Buffer.from([1]),
};

const getInitializeInstructionData = () => {
  return Buffer.concat([instructionDiscriminators.Initialize]);
};
const getSwitchPowerInstructionData = (name: string) => {
  return Buffer.concat([instructionDiscriminators.SwitchPower, encodeString(name, 64)]);
};

type PowerAccount = {
  is_on: boolean;
};

type PowerAccountRaw = {
  is_on: number;
};

const powerAccountSchema: borsh.Schema = {
  struct: {
    discriminator: 'u64',
    is_on: 'u8',
  },
};

// Define the functions to deserialize the account data read from the account
const deserializeDataAccount = (data: PowerAccountRaw): PowerAccount => {
  return {
    is_on: data.is_on === 1,
  };
};

describe('Account Data Program', () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  const powerAccount = Keypair.generate();

  before(async () => {
    context = await start(
      [
        { name: 'lever_program', programId: LEVER_PROGRAM_ID },
        { name: 'hand_program', programId: PROGRAM_ID },
      ],
      [],
    );
    client = context.banksClient;
    payer = context.payer;
  });

  it('Should init power account successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: LEVER_PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: powerAccount.publicKey, isSigner: true, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getInitializeInstructionData(),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer, powerAccount);

    // process the transaction
    await client.processTransaction(tx);

    const accountInfo = await client.getAccount(powerAccount.publicKey);
    assert.isNotNull(accountInfo);
    const rawAccountData = borsh.deserialize(powerAccountSchema, accountInfo?.data) as PowerAccountRaw;

    const deserializedData = deserializeDataAccount(rawAccountData);
    assert.isFalse(deserializedData.is_on);
  });

  it.skip('Should switch power by hand status successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [{ pubkey: powerAccount.publicKey, isSigner: true, isWritable: true }],
        data: getSwitchPowerInstructionData('Leo Pham'),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer, powerAccount);

    // process the transaction
    await client.processTransaction(tx);

    const accountInfo = await client.getAccount(powerAccount.publicKey);
    assert.isNotNull(accountInfo);
    const rawAccountData = borsh.deserialize(powerAccountSchema, accountInfo?.data) as PowerAccountRaw;
    const deserializedData = deserializeDataAccount(rawAccountData);
    assert.isTrue(deserializedData.is_on);
  });
});
