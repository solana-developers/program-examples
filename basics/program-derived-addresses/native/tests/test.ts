import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

describe('PDAs', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'program_derived_addresses_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;
  const rent = await client.getRent();

  class Assignable {
    constructor(properties) {
      for (const [key, value] of Object.entries(properties)) {
        this[key] = value;
      }
    }
  }

  class PageVisits extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(PageVisitsSchema, this));
    }

    static fromBuffer(buffer: Buffer) {
      return borsh.deserialize(PageVisitsSchema, PageVisits, buffer);
    }
  }
  const PageVisitsSchema = new Map([
    [
      PageVisits,
      {
        kind: 'struct',
        fields: [
          ['page_visits', 'u32'],
          ['bump', 'u8'],
        ],
      },
    ],
  ]);

  class IncrementPageVisits extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(IncrementPageVisitsSchema, this));
    }
  }
  const IncrementPageVisitsSchema = new Map([
    [
      IncrementPageVisits,
      {
        kind: 'struct',
        fields: [],
      },
    ],
  ]);

  const testUser = Keypair.generate();

  test('Create a test user', async () => {
    const ix = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      lamports: Number(rent.minimumBalance(BigInt(0))),
      newAccountPubkey: testUser.publicKey,
      programId: SystemProgram.programId,
      space: 0,
    });

    const tx = new Transaction();
    const blockhash = context.lastBlockhash;
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, testUser); // Add instruction and Sign the transaction

    await client.processTransaction(tx);
    console.log(`Local Wallet: ${payer.publicKey}`);
    console.log(`Created User: ${testUser.publicKey}`);
  });

  function derivePageVisitsPda(userPubkey: PublicKey) {
    return PublicKey.findProgramAddressSync([Buffer.from('page_visits'), userPubkey.toBuffer()], PROGRAM_ID);
  }

  test('Create the page visits tracking PDA', async () => {
    const [pageVisitsPda, pageVisitsBump] = derivePageVisitsPda(testUser.publicKey);
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: pageVisitsPda, isSigner: false, isWritable: true },
        { pubkey: testUser.publicKey, isSigner: false, isWritable: false },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: new PageVisits({ page_visits: 0, bump: pageVisitsBump }).toBuffer(),
    });
    const tx = new Transaction();
    const blockhash = context.lastBlockhash;
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });

  test('Visit the page!', async () => {
    const [pageVisitsPda, _] = derivePageVisitsPda(testUser.publicKey);
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: pageVisitsPda, isSigner: false, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      ],
      programId: PROGRAM_ID,
      data: new IncrementPageVisits({}).toBuffer(),
    });
    const tx = new Transaction();
    const blockhash = context.lastBlockhash;
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });

  test('Visit the page!', async () => {
    const [pageVisitsPda, _] = derivePageVisitsPda(testUser.publicKey);
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: pageVisitsPda, isSigner: false, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      ],
      programId: PROGRAM_ID,
      data: new IncrementPageVisits({}).toBuffer(),
    });
    const tx = new Transaction();
    const [blockhash, _block_height] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });

  test('Read page visits', async () => {
    const [pageVisitsPda, _] = derivePageVisitsPda(testUser.publicKey);
    const accountInfo = await client.getAccount(pageVisitsPda);
    const readPageVisits = PageVisits.fromBuffer(Buffer.from(accountInfo.data));
    console.log(`Number of page visits: ${readPageVisits.page_visits}`);
  });
});
