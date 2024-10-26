import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { assert } from 'chai';
import { describe, it } from 'mocha';
import { BanksClient, ProgramTestContext, start } from 'solana-bankrun';

const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');

// Define the instruction discriminators
const instructionDiscriminators = {
  TransferSolWithCpi: Buffer.from([0]),
  TransferSolWithProgram: Buffer.from([1]),
};

const getTransferSolWithCpiInstructionData = (amount: number) => {
  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(BigInt(amount));
  return Buffer.concat([instructionDiscriminators.TransferSolWithCpi, Uint8Array.from(buffer)]);
};

const getTransferSolWithProgramInstructionData = (amount: number) => {
  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(BigInt(amount));
  return Buffer.concat([instructionDiscriminators.TransferSolWithProgram, Uint8Array.from(buffer)]);
};

describe('Transfer Sol Program', () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  let recipient: Keypair;
  let transferAmount = 1000_000_000;
  let programOwnedAccount: Keypair;
  before(async () => {
    context = await start([{ name: 'transfer_sol_program', programId: PROGRAM_ID }], []);
    client = context.banksClient;
    payer = context.payer;
    recipient = Keypair.generate();
    programOwnedAccount = Keypair.generate();

    // create the program owned account for testing transfer sol with program
    const createAccountIx = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: programOwnedAccount.publicKey,
      lamports: transferAmount,
      space: 0,
      programId: PROGRAM_ID,
    });

    let tx = new Transaction();
    tx.add(createAccountIx);
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer, programOwnedAccount);

    await client.processTransaction(tx);
  });

  it('Should transfer SOL using CPI successfully', async () => {
    const recipientPreBalance = await client.getBalance(recipient.publicKey);
    // send the transaction
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: recipient.publicKey, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: getTransferSolWithCpiInstructionData(transferAmount),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    // check the balances
    const recipientPostBalance = await client.getBalance(recipient.publicKey);
    assert.equal(recipientPostBalance, recipientPreBalance + BigInt(transferAmount));
  });

  it('Should transfer SOL using program successfully', async () => {
    const recipientPreBalance = await client.getBalance(recipient.publicKey);
    // send the transaction
    const tx = new Transaction();
    tx.add(
      new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
          {
            pubkey: programOwnedAccount.publicKey,
            isSigner: false,
            isWritable: true,
          },
          { pubkey: recipient.publicKey, isSigner: false, isWritable: true },
        ],
        data: getTransferSolWithProgramInstructionData(transferAmount),
      }),
    );
    tx.recentBlockhash = context.lastBlockhash;
    tx.sign(payer);

    // process the transaction
    await client.processTransaction(tx);

    // check the balances
    const recipientPostBalance = await client.getBalance(recipient.publicKey);
    assert.equal(recipientPostBalance, recipientPreBalance + BigInt(transferAmount));
  });
});
