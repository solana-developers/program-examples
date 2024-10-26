import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

const instructionDiscriminators = {
  withCpi: Buffer.from([0]),
  withProgram: Buffer.from([1]),
};

describe('transfer sol', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;

  before(async () => {
    context = await start([{ name: 'transfer_sol_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
  });

  it('should transfer between accounts using our program', async () => {
    // 1 SOL
    const amount = 1 * LAMPORTS_PER_SOL;

    // Generate a couple of keypairs to create accounts owned by our program
    const acc1 = Keypair.generate();
    const acc2 = Keypair.generate();

    const createAccountIx = (pubkey: PublicKey) => {
      return SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: pubkey,
        space: 0,
        lamports: 2 * LAMPORTS_PER_SOL,
        programId: PROGRAM_ID,
      });
    };

    // Create the accounts
    const createAccountsTx = new Transaction();
    createAccountsTx.recentBlockhash = context.lastBlockhash;
    createAccountsTx.add(createAccountIx(acc1.publicKey)).add(createAccountIx(acc2.publicKey)).sign(payer, acc1, acc2);

    await client.processTransaction(createAccountsTx);

    // Prepare the transfer instruction
    const amountBuffer = Buffer.alloc(8);
    amountBuffer.writeBigInt64LE(BigInt(amount), 0);

    const data = Buffer.concat([instructionDiscriminators.withProgram, amountBuffer]);

    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: acc1.publicKey, isSigner: false, isWritable: true },
        { pubkey: acc2.publicKey, isSigner: false, isWritable: true },
      ],
      data,
    });

    // Prepare the transaction
    const [blockHash, _] = await client.getLatestBlockhash();
    const tx = new Transaction();
    tx.recentBlockhash = blockHash;
    tx.add(ix).sign(payer);

    // Check initial balance
    const initialRecipientBalance = await client.getBalance(acc2.publicKey);

    // Execute the transaction
    await client.processTransaction(tx);

    // Check the balance
    const finalRecipientBalance = await client.getBalance(acc2.publicKey);
    assert.equal(finalRecipientBalance, initialRecipientBalance + BigInt(amount));
  });

  it('should transfer between accounts using the system programa', async () => {
    // 1 SOL
    const amount = 1 * LAMPORTS_PER_SOL;

    // Generate a new keypair for the recipient
    const recipient = Keypair.generate();

    const amountBuffer = Buffer.alloc(8);
    amountBuffer.writeBigInt64LE(BigInt(amount), 0);

    // Prepare the instruction
    const data = Buffer.concat([instructionDiscriminators.withCpi, amountBuffer]);

    console.log(data);

    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: recipient.publicKey, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data,
    });

    // Prepare the transaction
    const tx = new Transaction();
    tx.recentBlockhash = context.lastBlockhash;
    tx.add(ix).sign(payer);

    // Execute the transaction
    await client.processTransaction(tx);

    // Check the balance
    const recipientBalance = await client.getBalance(recipient.publicKey);
    assert.equal(recipientBalance, BigInt(amount));
  });
});
