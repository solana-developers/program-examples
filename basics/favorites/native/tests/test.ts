import assert from 'node:assert';
import { Buffer } from 'node:buffer';
import { describe, it } from 'node:test';
import * as web3 from '@solana/web3.js';
import BN from 'bn.js';
import * as borsh from 'borsh';
import { expect } from 'chai';
import { start } from 'solana-bankrun';

class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

class FavoritesAccount extends Assignable {
  number: BN;
  color: string;
  hobbies: string[];
  toBuffer() {
    return Buffer.from(borsh.serialize(FavoritesAccountSchema, this));
  }
  static fromBuffer(buffer: Buffer) {
    return borsh.deserialize(FavoritesAccountSchema, FavoritesAccount, buffer);
  }
}
const FavoritesAccountSchema = new Map([
  [
    FavoritesAccount,
    {
      kind: 'struct',
      fields: [
        ['number', 'u64'],
        ['color', 'string'],
        ['hobbies', ['string']],
      ],
    },
  ],
]);

describe('Favorites Program(Native) Test', async () => {
  const PROGRAM_ID = web3.PublicKey.unique();
  const context = await start([{ name: 'favorites_program', programId: PROGRAM_ID }], []);

  const client = context.banksClient;
  const payer = context.payer;

  const getFavoritesPDA = () => {
    const [favoritesAccountPda, _] = web3.PublicKey.findProgramAddressSync([Buffer.from('favorites'), payer.publicKey.toBuffer()], PROGRAM_ID);
    return favoritesAccountPda;
  };

  const createFavoritesTransaction = async (number: BN, color: string, hobbies: string[]) => {
    const favoritesAccount = new FavoritesAccount({
      number,
      color,
      hobbies,
    }).toBuffer();
    const instructionData = Buffer.concat([favoritesAccount]);

    const setFavoritesIx = new web3.TransactionInstruction({
      keys: [
        {
          pubkey: payer.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: getFavoritesPDA(),
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: web3.SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: PROGRAM_ID,
      data: instructionData,
    });

    const tx = new web3.Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.feePayer = payer.publicKey;
    tx.add(setFavoritesIx).sign(payer);
    return tx;
  };

  const validateFavoritesAccount = async (number: BN, color: string, hobbies: string[]) => {
    const favoritesAccountInfo = await client.getAccount(getFavoritesPDA());
    if (favoritesAccountInfo) {
      const deserializedAccount = FavoritesAccount.fromBuffer(Buffer.from(favoritesAccountInfo.data));
      expect(deserializedAccount.color).to.eq(color);
      expect(deserializedAccount.number.eq(number), 'Expected number to equal');
      assert.deepStrictEqual(deserializedAccount.hobbies, hobbies, 'hobbies mismatch');
    } else {
      console.log('Account not found');
    }
  };

  it('Sets Favorites for the first time', async () => {
    const number = new BN(52);
    const color = 'blue';
    const hobbies = ['hobby1'];

    const tx = await createFavoritesTransaction(number, color, hobbies);

    await client.processTransaction(tx);

    await validateFavoritesAccount(number, color, hobbies);
  });

  it('Updates Favorites', async () => {
    const number = new BN(248);
    const color = 'violet';
    const hobbies = ['hobby1', 'hobby2'];

    const tx = await createFavoritesTransaction(number, color, hobbies);
    await client.processTransaction(tx);

    await validateFavoritesAccount(number, color, hobbies);
  });
});
