import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { BN } from "bn.js";
import * as borsh from "borsh";
import { assert, expect } from "chai";
import { before, beforeEach, describe, test } from 'node:test';
import { LiteSVM } from 'litesvm';

// This is a helper class to assign properties to the class
class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

const MyInstruction = {
  CreateFav: 0,
  GetFav: 1,
} as const;

class CreateFav extends Assignable {
  number: number;
  instruction: number;
  color: string;
  hobbies: string[];

  toBuffer() {
    return Buffer.from(borsh.serialize(CreateNewAccountSchema, this));
  }

  static fromBuffer(buffer: Buffer): CreateFav {
    return borsh.deserialize(
      {
        struct: {
          number: "u64",
          color: "string",
          hobbies: {
            array: {
              type: "string",
            },
          },
        },
      },
      buffer,
    ) as CreateFav;
  }
}
const CreateNewAccountSchema = {
  struct: {
    instruction: "u8",
    number: "u64",
    color: "string",
    hobbies: {
      array: {
        type: "string",
      },
    },
  },
};

class GetFav extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(GetFavSchema, this));
  }
}
const GetFavSchema = {
  struct: {
    instruction: "u8",
  },
};

describe("Favorites Solana Native", () => {
  const programId = PublicKey.unique();

  let svm: LiteSVM;
  let payer: Keypair;
  let blockhash: string;

  beforeEach(() => {
    svm = new LiteSVM();
    svm.addProgramFromFile(programId, 'tests/fixtures/favorites_native.so');
    payer = Keypair.generate();
    svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));
    blockhash = svm.latestBlockhash();
  });

  test("Set the favorite pda and cross-check the updated data", () => {
    const favoritesPda = PublicKey.findProgramAddressSync(
      [Buffer.from("favorite"), payer.publicKey.toBuffer()],
      programId,
    )[0];
    const favData = {
      instruction: MyInstruction.CreateFav,
      number: 42,
      color: "blue",
      hobbies: ["coding", "reading", "traveling"],
    };
    const favorites = new CreateFav(favData);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: favoritesPda, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId,
      data: favorites.toBuffer(),
    });

    const tx = new Transaction().add(ix);

    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = blockhash;
    tx.sign(payer);
    tx.recentBlockhash = blockhash;
    svm.sendTransaction(tx);

    const account = svm.getAccount(favoritesPda);
    const data = Buffer.from(account.data);

    const favoritesData = CreateFav.fromBuffer(data);

    console.log("Deserialized data:", favoritesData);

    // biome-ignore lint/suspicious/noExplicitAny: borsh deserialization returns dynamic types
    expect(new BN(favoritesData.number as any, "le").toNumber()).to.equal(
      favData.number,
    );
    expect(favoritesData.color).to.equal(favData.color);
    expect(favoritesData.hobbies).to.deep.equal(favData.hobbies);
  });

  test("Check if the test fails if the pda seeds aren't same", () => {
    const favoritesPda = PublicKey.findProgramAddressSync(
      [Buffer.from("favorite"), payer.publicKey.toBuffer()],
      programId,
    )[0];
    const favData = {
      instruction: MyInstruction.CreateFav,
      number: 42,
      color: "blue",
      hobbies: ["coding", "reading", "traveling"],
    };
    const favorites = new CreateFav(favData);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: favoritesPda, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId,
      data: favorites.toBuffer(),
    });

    const tx = new Transaction().add(ix);

    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = blockhash;
    tx.sign(payer);
    tx.recentBlockhash = blockhash;
    try {
      svm.sendTransaction(tx);
      console.error("Expected the test to fail");
    } catch (_err) {
      assert(true);
    }
  });

  test("Get the favorite pda and cross-check the data", () => {
    const favoritesPda = PublicKey.findProgramAddressSync(
      [Buffer.from("favorite"), payer.publicKey.toBuffer()],
      programId,
    )[0];
    const favData = {
      instruction: MyInstruction.CreateFav,
      number: 42,
      color: "hazel",
      hobbies: ["singing", "dancing", "skydiving"],
    };
    const favorites = new CreateFav(favData);

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: favoritesPda, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId,
      data: favorites.toBuffer(),
    });

    const tx1 = new Transaction().add(ix);

    tx1.feePayer = payer.publicKey;
    tx1.recentBlockhash = blockhash;
    tx1.sign(payer);
    tx1.recentBlockhash = blockhash;
    svm.sendTransaction(tx1);

    // Getting the user's data through the get_pda instruction
    const getfavData = { instruction: MyInstruction.GetFav };
    const getfavorites = new GetFav(getfavData);

    const ix2 = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: favoritesPda, isSigner: false, isWritable: false },
      ],
      programId,
      data: getfavorites.toBuffer(),
    });

    const tx = new Transaction().add(ix2);

    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = blockhash;
    tx.sign(payer);
    tx.recentBlockhash = blockhash;
    svm.sendTransaction(tx);
  });
});
