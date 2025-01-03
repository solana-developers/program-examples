import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';

import { ProgramTestContext, BanksClient, start } from 'solana-bankrun';
import * as borsh from 'borsh';
import { describe, it } from 'mocha';
import { assert } from 'chai';
const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

export const encodeString = (str: string, length: number): Uint8Array => {
  const buffer = Buffer.alloc(length, 0);
  buffer.write(str, 'utf-8');
  return Uint8Array.from(buffer);
};

export const decodeString = (data: Uint8Array): string => {
  return Buffer.from(data).toString('utf-8').replace(/\0/g, '');
};
const instructionDiscriminators = {
  CreateUser: Buffer.from([0]),
  CloseUser: Buffer.from([1]),
};

const getCloseUserInstructionData = () => {
  return Buffer.concat([instructionDiscriminators.CloseUser]);
};
const getCreateUserInstructionData = (name: string) => {
  return Buffer.concat([instructionDiscriminators.CreateUser, encodeString(name, 64)]);
};

type UserStateAccount = {
  bump: number;
  user: PublicKey;
  name: string;
};

type UserStateAccountRaw = {
  bump: number;
  user: Uint8Array;
  name: Uint8Array;
};

const userStateAccountSchema: borsh.Schema = {
  struct: {
    discriminator: 'u64',
    bump: 'u8',
    user: { array: { type: 'u8', len: 32 } },
    name: { array: { type: 'u8', len: 64 } },
  },
};

// Define the functions to deserialize the account data read from the account
const deserializeDataAccount = (data: UserStateAccountRaw): UserStateAccount => {
  return {
    user: new PublicKey(data.user),
    name: decodeString(data.name),
    bump: data.bump,
  };
};

describe('Close account Program', () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  let userPda: PublicKey;
  let bump: number;

  before(async () => {
    context = await start([{ name: 'close_account_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;

    [userPda, bump] = PublicKey.findProgramAddressSync([Buffer.from('USER'), payer.publicKey.toBuffer()], PROGRAM_ID);
  });

  it('Should create user state account successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: userPda, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getCreateUserInstructionData('Leo Pham'),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    const accountInfo = await client.getAccount(userPda);
    assert.isNotNull(accountInfo);
    const rawAccountData = borsh.deserialize(userStateAccountSchema, accountInfo?.data) as UserStateAccountRaw;

    const deserializedData = deserializeDataAccount(rawAccountData);
    assert.equal(deserializedData.name, 'Leo Pham');
    assert.equal(deserializedData.bump, bump);
    assert.equal(deserializedData.user.toBase58(), payer.publicKey.toBase58());
  });

  it('Should close user state account successfully', async () => {
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: userPda, isSigner: false, isWritable: true },
        ],
        data: getCloseUserInstructionData(),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    const accountInfo = await client.getAccount(userPda);
    assert.isNull(accountInfo);
  });
});
