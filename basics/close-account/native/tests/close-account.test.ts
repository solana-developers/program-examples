import { describe, test } from 'node:test';
import { PublicKey, Transaction } from '@solana/web3.js';
import { expect } from 'chai';
import { start } from 'solana-bankrun';
import { createCloseUserInstruction, createCreateUserInstruction } from '../ts';

describe('Close Account!', async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start([{ name: 'close_account_native_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  const testAccountPublicKey = PublicKey.findProgramAddressSync([Buffer.from('USER'), payer.publicKey.toBuffer()], PROGRAM_ID)[0];

  test('Create the account', async () => {
    const blockhash = context.lastBlockhash;
    const ix = createCreateUserInstruction(testAccountPublicKey, payer.publicKey, PROGRAM_ID, 'Jacob');

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);
  });

  test('Close the account', async () => {
    const blockhash = context.lastBlockhash;

    const ix = createCloseUserInstruction(testAccountPublicKey, payer.publicKey, PROGRAM_ID);
    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    const testAccountInitialBalance = Number(await client.getBalance(testAccountPublicKey));
    const payerInitialBalance = Number(await client.getBalance(payer.publicKey));
    await client.processTransaction(tx);
    const payerFinalBalance = Number(await client.getBalance(payer.publicKey));
    const testAccountFinalBalance = Number(await client.getBalance(testAccountPublicKey));

    expect(payerFinalBalance).to.be.greaterThan(payerInitialBalance); // Assumes rent was > tx fee
    expect(testAccountInitialBalance).to.be.greaterThan(0);
    expect(testAccountFinalBalance).to.equal(0);
  });
});
