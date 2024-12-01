import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';

describe('Create a system account', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start([{ name: 'create_account_steel_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  test('Create the account via a cross program invocation', async () => {
    const newKeypair = Keypair.generate();
    const blockhash = context.lastBlockhash;

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: newKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from([0]), // instruction discriminator
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, newKeypair);

    await client.processTransaction(tx);

    const rent = await client.getRent();
    // Minimum balance for rent exemption for new account
    const lamports = rent.minimumBalance(BigInt(0));

    // Check that the account was created
    const accountInfo = await client.getAccount(newKeypair.publicKey);
    assert(BigInt(accountInfo.lamports) === lamports);
    assert(accountInfo.owner.toBase58() === SystemProgram.programId.toBase58());
  });

  test('Create the account via direct call to system program', async () => {
    const newKeypair = Keypair.generate();
    const blockhash = context.lastBlockhash;

    const rent = await client.getRent();
    // Minimum balance for rent exemption for new account
    const lamports = rent.minimumBalance(BigInt(0));

    const ix = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: newKeypair.publicKey,
      lamports: Number(lamports),
      space: 0,
      programId: SystemProgram.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, newKeypair);

    await client.processTransaction(tx);
    console.log(`Account with public key ${newKeypair.publicKey} successfully created`);

    // Check that the account was created
    const accountInfo = await client.getAccount(newKeypair.publicKey);
    assert(BigInt(accountInfo.lamports) === lamports);
    assert(accountInfo.owner.toBase58() === SystemProgram.programId.toBase58());
  });
});
