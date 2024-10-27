import { describe, test } from 'mocha';
import { assert, expect } from 'chai';
import { Blockhash, Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BanksClient, ProgramTestContext, Rent, start , } from 'solana-bankrun';
import { BN } from 'bn.js';
import * as borsh from "borsh"


class Assignable {
  constructor(properties) {
    for (const [key, value] of Object.entries(properties)) {
      this[key] = value;
    }
  }
}

enum MyInstruction {
  CreateFav = 0,
  GetFav = 1,
}

class CreateFav extends Assignable {

  number: number;
  instruction: MyInstruction;
  color: string;
  hobbies: string[];

  toBuffer() {
    return Buffer.from(borsh.serialize(CreateNewAccountSchema, this));
  }

  static fromBuffer(buffer: Buffer): CreateFav {
    return borsh.deserialize({
      struct: {
        number: "u64",
        color: "string",
        hobbies: {
          array: {
            type: "string"
          }
        }
      }}, buffer) as CreateFav;
  }
}
const CreateNewAccountSchema = {
  "struct": {
    instruction: "u8",
    number: "u64",
    color: "string",
    hobbies: {
      array: {
        type: "string"
      }
    }
  }
}

class GetFav extends Assignable {
  toBuffer() {
    return Buffer.from(borsh.serialize(GetFavSchema, this));
  }
}
const GetFavSchema = {
  "struct": {
    instruction: "u8"
  }
}

describe('Favorites Solana Native',  () => {
  
// Randomly generate the program keypair and load the program to solana-bankrun
const programId = PublicKey.unique();

let context: ProgramTestContext, client:BanksClient, payer: Keypair, blockhash: Blockhash; 
beforeEach(async () => {
  context = await start([{ name: 'favorites_native', programId }], []);
  client = context.banksClient;
  // Get the payer keypair from the context, this will be used to sign transactions with enough lamports
  payer = context.payer;
  blockhash = context.lastBlockhash;
})
  
test('Set the favorite pda and cross-check the updated data', async () => {
    const favoritesPda = PublicKey.findProgramAddressSync([Buffer.from("favorite"), payer.publicKey.toBuffer()], programId)[0];
    const favData = {instruction: MyInstruction.CreateFav, number: 42, color: "blue", hobbies: ["coding", "reading", "traveling"]}
    const favorites = new CreateFav(favData);

    const ix = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true}, 
        {pubkey: favoritesPda, isSigner: false, isWritable: true}, 
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}],
      programId,
      data: favorites.toBuffer(),
    });

    const tx = new Transaction().add(ix);

    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = blockhash
    tx.sign(payer)
    tx.recentBlockhash = blockhash;
    await client.processTransaction(tx);

    const account = await client.getAccount(favoritesPda);
    const data = Buffer.from(account.data);

    const favoritesData = CreateFav.fromBuffer(data);

    console.log("Deserialized data:", favoritesData);

    expect(new BN(favoritesData.number as any, 'le').toNumber()).to.equal(favData.number);
    expect(favoritesData.color).to.equal(favData.color);
    expect(favoritesData.hobbies).to.deep.equal(favData.hobbies);
  });

  test('Check if the test fails if the pda seeds aren\'t same', async () => {
    // We put the wrong seeds knowingly to see if the test fails because of checks
    const favoritesPda = PublicKey.findProgramAddressSync([Buffer.from("favorite"), payer.publicKey.toBuffer()], programId)[0];
    const favData = {instruction: MyInstruction.CreateFav, number: 42, color: "blue", hobbies: ["coding", "reading", "traveling"]}
    const favorites = new CreateFav(favData);

    const ix = new TransactionInstruction({
      keys: [{pubkey: payer.publicKey, isSigner: true, isWritable: true}, {pubkey: favoritesPda, isSigner: false, isWritable: true}, {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}],
      programId,
      data: favorites.toBuffer(),
    });

    const tx = new Transaction().add(ix);

    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = blockhash
    tx.sign(payer)
    tx.recentBlockhash = blockhash;
    try {
      await client.processTransaction(tx)
      console.error("Expected the test to fail")
    } catch(err) {
      assert(true)
    }
  });

  test('Get the favorite pda and cross-check the data', async () => {
    // Creating a new account with payer's pubkey
    const favoritesPda = PublicKey.findProgramAddressSync([Buffer.from("favorite"), payer.publicKey.toBuffer()], programId)[0];
    const favData = {instruction: MyInstruction.CreateFav, number: 42, color: "hazel", hobbies: ["singing", "dancing", "skydiving"]}
    const favorites = new CreateFav(favData);
    
    const ix = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true}, 
        {pubkey: favoritesPda, isSigner: false, isWritable: true}, 
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}],
      programId,
      data: favorites.toBuffer(),
    });

    const tx1 = new Transaction().add(ix);
    
    tx1.feePayer = payer.publicKey;
    tx1.recentBlockhash = blockhash
    tx1.sign(payer)
    tx1.recentBlockhash = blockhash;
    await client.processTransaction(tx1)


    // Getting the user's data through the get_pda instruction
    const getfavData = {instruction: MyInstruction.GetFav}
    const getfavorites = new GetFav(getfavData);
    
    const ix2 = new TransactionInstruction({
      keys: [{pubkey: payer.publicKey, isSigner: true, isWritable: true}, {pubkey: favoritesPda, isSigner: false, isWritable: false}],
      programId,
      data:getfavorites.toBuffer(),
    }); 

    const tx = new Transaction().add(ix2);

    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = blockhash
    tx.sign(payer)
    tx.recentBlockhash = blockhash;
    await client.processTransaction(tx);
  })
})