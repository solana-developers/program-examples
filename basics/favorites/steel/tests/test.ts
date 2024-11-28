import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

type FavoritesData = {
  number: bigint;
  color: string;
  hobbies: string[];
};

class Favorites {
  number: bigint;
  color: Uint8Array;
  hobbies: Uint8Array[];

  constructor(props: {
    number: bigint;
    color: Uint8Array;
    hobbies: Uint8Array[];
  }) {
    this.number = props.number;
    this.color = props.color;
    this.hobbies = props.hobbies;
  }

  toBuffer() {
    return Buffer.from(borsh.serialize(FavoritesSchema, this));
  }

  static fromAccountData(buffer: Buffer) {
    const _accountData = Uint8Array.from(buffer).slice(8); // remove 8 byte discriminator

    return borsh.deserialize(FavoritesSchema, Favorites, Buffer.from(_accountData));
  }

  static fromBuffer(buffer: Buffer) {
    const _buffer = Uint8Array.from(buffer).slice(8);

    return borsh.deserialize(FavoritesSchema, Favorites, Buffer.from(_buffer));
  }

  static fromInfo(info: { number: number; color: string; hobbies: string[] }) {
    return new Favorites({
      number: BigInt(info.number),
      color: Uint8Array.from(Buffer.from(info.color.padEnd(48, '\0'))),
      hobbies: info.hobbies.map((hobby) => Uint8Array.from(Buffer.from(hobby.padEnd(48, '\0')))),
    });
  }

  toData(): FavoritesData {
    return {
      number: BigInt(this.number),
      color: Buffer.from(this.color).toString(),
      hobbies: this.hobbies.map((hobby) => Buffer.from(hobby).toString()),
    };
  }
}

const FavoritesSchema = new Map([
  [
    Favorites,
    {
      kind: 'struct',
      fields: [
        ['number', 'u64'],
        ['color', [48]], // Fixed array of 48 bytes
        ['hobbies', [[48], 5]], // Fixed array of 5 48 bytes array
      ],
    },
  ],
]);

describe('Favorites!', async () => {
  const favoritesAccount = Keypair.generate();
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start([{ name: 'favorites_steel_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;

  function favoritesPda(userPubkey: PublicKey) {
    return PublicKey.findProgramAddressSync([Buffer.from('favorites'), userPubkey.toBuffer()], PROGRAM_ID);
  }

  test('Set the favorites data', async () => {
    const payer = context.payer;

    console.log(`Program Address : ${PROGRAM_ID}`);
    console.log(`Payer Address   : ${payer.publicKey}`);
    console.log(`Favorites Acct  : ${favoritesAccount.publicKey}`);

    const favorites = Favorites.fromInfo({
      number: 2,
      color: 'green',
      hobbies: ['singing', 'reading', 'jogging', 'dancing', 'traveling'],
    });

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: favoritesPda(payer.publicKey)[0],
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: payer.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.concat([
        Buffer.from([0]), // SetFavorites Discriminator
        favorites.toBuffer(),
      ]),
    });

    const blockhash = context.lastBlockhash;

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);
    await client.processTransaction(tx);
  });

  test("Read favorites's data", async () => {
    const accountInfo = await client.getAccount(favoritesPda(context.payer.publicKey)[0]);

    const readFavorites = Favorites.fromAccountData(Buffer.from(accountInfo.data)).toData();

    console.log(`number : ${readFavorites.number}`);
    console.log(`color  : ${readFavorites.color}`);
    console.log(`Hobbies: ${readFavorites.hobbies}`);
  });
});
