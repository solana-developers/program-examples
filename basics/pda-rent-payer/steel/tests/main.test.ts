import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

const instructionDiscriminators = {
  withCpi: Buffer.from([0]),
  withProgram: Buffer.from([1]),
};

describe('pda-rent-payer', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;

  before(async () => {
    context = await start([{ name: 'transfer_sol_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('should create an account with PDA as rent payer', async () => {
    // 1 SOL
    const amount = 1 * LAMPORTS_PER_SOL;

    // Generate a PDA
    const [pda, bump] = await PublicKey.findProgramAddress(
      [Buffer.from('example_seed')],
      PROGRAM_ID
    );

    // Fund the PDA with some SOL
    const fundPdaIx = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: pda,
      lamports: 2 * LAMPORTS_PER_SOL,
    });

    const fundPdaTx = new Transaction();
    fundPdaTx.recentBlockhash = context.lastBlockhash;
    fundPdaTx.add(fundPdaIx).sign(payer);

    await client.processTransaction(fundPdaTx);

    // Generate a new keypair for the account
    const newAccount = Keypair.generate();

    // Prepare the instruction to create an account with PDA as rent payer
    const data = instructionDiscriminators.withProgram;

    const createAccountWithPdaIx = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: pda, isSigner: false, isWritable: true },
        { pubkey: newAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data,
    });

    const createAccountWithPdaTx = new Transaction();
    createAccountWithPdaTx.recentBlockhash = context.lastBlockhash;
    createAccountWithPdaTx.add(createAccountWithPdaIx).sign(payer, newAccount);

    // Execute the transaction
    await client.processTransaction(createAccountWithPdaTx);

    // Check the balance of the new account
    const newAccountBalance = await client.getBalance(newAccount.publicKey);
    assert.equal(newAccountBalance, BigInt(amount));
  });
});
