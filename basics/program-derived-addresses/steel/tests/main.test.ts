import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

type PageVisits = {
  page_visits: number;
  bump: number;
};

const pageVisitsSchema: borsh.Schema = {
  struct: {
    discriminator: 'u64',
    page_visits: 'u32',
    bump: 'u8',
  },
};

const createPageVisitsBuffer = (data: PageVisits): Buffer => {
  const pageVisits = Buffer.alloc(4);
  pageVisits.writeUInt32LE(data.page_visits, 0);
  const bump = Buffer.alloc(1);
  bump.writeUInt8(data.bump, 0);
  return Buffer.concat([pageVisits, bump]);
};

describe('program derived addresses program', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;

  const testUser = Keypair.generate();

  const instructionDiscriminators = {
    create: Buffer.from([0]),
    increment: Buffer.from([1]),
  };

  const derivePageVisitsPDA = async (user: PublicKey) => {
    const seed = Buffer.from('program-derived-addresses');
    return PublicKey.findProgramAddressSync([seed, user.toBuffer()], PROGRAM_ID);
  };

  before(async () => {
    context = await start([{ name: 'program_derived_addresses_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('should create a page visits tracking PDA', async () => {
    const [pageVisitsPDA] = await derivePageVisitsPDA(testUser.publicKey);

    // create the page visits data
    const pageVisits: PageVisits = { page_visits: 0, bump: 0 };
    const pageVisitsBuffer = createPageVisitsBuffer(pageVisits);
    const data = Buffer.concat([instructionDiscriminators.create, pageVisitsBuffer]);

    // create the create instruction
    const createIx = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: testUser.publicKey, isSigner: false, isWritable: false },
        { pubkey: pageVisitsPDA, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data,
    });

    // send the create transaction
    const createTx = new Transaction();
    createTx.recentBlockhash = context.lastBlockhash;
    createTx.add(createIx).sign(payer);

    // process the transaction
    await client.processTransaction(createTx);

    // fetch the counter account data
    const pageVisitsInfo = await client.getAccount(pageVisitsPDA);
    assert(pageVisitsInfo !== null, 'account should exist');
  });

  it('should visit the page and get 1 visit!', async () => {
    const [pageVisitsPDA] = await derivePageVisitsPDA(testUser.publicKey);

    // create the increment instruction
    const incrementIx = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [{ pubkey: pageVisitsPDA, isSigner: false, isWritable: true }],
      data: instructionDiscriminators.increment,
    });

    // send the increment transaction
    const incrementTx = new Transaction();
    incrementTx.recentBlockhash = context.lastBlockhash;
    incrementTx.add(incrementIx).sign(payer);

    // process the transaction
    await client.processTransaction(incrementTx);

    // fetch the account data
    const pageVisitsInfo = await client.getAccount(pageVisitsPDA);
    assert(pageVisitsInfo !== null, 'account should exist');

    const data = borsh.deserialize(pageVisitsSchema, pageVisitsInfo?.data) as PageVisits;

    assert(data.page_visits === 1, 'page visits should be 1');
  });

  it('should visit the page and get 2 visits!', async () => {
    const [pageVisitsPDA] = await derivePageVisitsPDA(testUser.publicKey);

    // create the increment instruction
    const incrementIx = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [{ pubkey: pageVisitsPDA, isSigner: false, isWritable: true }],
      data: instructionDiscriminators.increment,
    });

    // get last blockhash
    const [blockhash, _block_height] = await client.getLatestBlockhash();

    // send the increment transaction
    const incrementTx = new Transaction();
    incrementTx.recentBlockhash = blockhash;
    incrementTx.add(incrementIx).sign(payer);

    // process the transaction
    await client.processTransaction(incrementTx);

    // fetch the account data
    const pageVisitsInfo = await client.getAccount(pageVisitsPDA);
    assert(pageVisitsInfo !== null, 'account should exist');

    const data = borsh.deserialize(pageVisitsSchema, pageVisitsInfo?.data) as PageVisits;

    assert(data.page_visits === 2, 'page visits should be 2');
  });

  it('should read all the visits of the page', async () => {
    const [pageVisitsPDA] = await derivePageVisitsPDA(testUser.publicKey);

    // fetch the account data
    const pageVisitsInfo = await client.getAccount(pageVisitsPDA);
    assert(pageVisitsInfo !== null, 'account should exist');

    const data = borsh.deserialize(pageVisitsSchema, pageVisitsInfo?.data) as PageVisits;

    assert(data.page_visits === 2, 'page visits should be 2');
  });
});
