import { describe, test } from 'node:test';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from '@solana/web3.js';
import { assert } from 'chai';
import { start } from 'solana-bankrun';
import { InstructionType, createTransferInstruction, createTransferWithProgramInstruction } from './instruction';

describe('transfer-sol', async () => {
  const PROGRAM_ID = new PublicKey('z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35');
  const context = await start([{ name: 'transfer_sol_steel_program', programId: PROGRAM_ID }], []);
  const client = context.banksClient;
  const payer = context.payer;

  const transferAmount = 1 * LAMPORTS_PER_SOL;
  const test1Recipient = Keypair.generate();
  const test2Recipient1 = Keypair.generate();
  const test2Recipient2 = Keypair.generate();

  test('Transfer between accounts using the system program', async () => {
    const [, recipientBalanceBefore] = await getBalances(payer.publicKey, test1Recipient.publicKey, 'Beginning');

    const ix = createTransferInstruction(payer.publicKey, test1Recipient.publicKey, PROGRAM_ID, InstructionType.CpiTransfer, transferAmount);

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    await client.processTransaction(tx);

    const [, recipientBalanceAfter] = await getBalances(payer.publicKey, test1Recipient.publicKey, 'Resulting');

    assert(recipientBalanceAfter === recipientBalanceBefore + BigInt(transferAmount));
  });

  test('Create two accounts for the following test', async () => {
    const ix = (pubkey: PublicKey) => {
      return SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: pubkey,
        space: 0,
        lamports: 2 * LAMPORTS_PER_SOL,
        programId: PROGRAM_ID,
      });
    };

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix(test2Recipient1.publicKey)).add(ix(test2Recipient2.publicKey)).sign(payer, test2Recipient1, test2Recipient2);

    await client.processTransaction(tx);
  });

  test('Transfer between accounts using our program', async () => {
    const [, recipientBalanceBefore] = await getBalances(test2Recipient1.publicKey, test2Recipient2.publicKey, 'Beginning');

    const ix = createTransferWithProgramInstruction(
      test2Recipient1.publicKey,
      test2Recipient2.publicKey,
      PROGRAM_ID,
      InstructionType.ProgramTransfer,
      transferAmount,
    );

    const tx = new Transaction();
    const [blockhash, _] = await client.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, test2Recipient1);

    await client.processTransaction(tx);

    const [, recipientBalanceAfter] = await getBalances(test2Recipient1.publicKey, test2Recipient2.publicKey, 'Resulting');

    assert(recipientBalanceAfter === recipientBalanceBefore + BigInt(transferAmount));
  });

  async function getBalances(payerPubkey: PublicKey, recipientPubkey: PublicKey, timeframe: string) {
    const payerBalance = await client.getBalance(payerPubkey);
    const recipientBalance = await client.getBalance(recipientPubkey);

    console.log(`${timeframe} balances:`);
    console.log(`   Payer: ${payerBalance}`);
    console.log(`   Recipient: ${recipientBalance}`);

    return [payerBalance, recipientBalance];
  }
});
