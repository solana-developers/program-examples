import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { AnchorProgramExample } from '../target/types/anchor_program_example';

const IDL = require('../target/idl/anchor_program_example.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun example', async () => {
  const context = await startAnchor('', [{ name: 'anchor_program_example', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<AnchorProgramExample>(IDL, provider);
  const client = context.banksClient;

  // We'll create this ahead of time.
  // Our program will try to modify it.
  const accountToChange = new Keypair();
  // Our program will create this.
  const accountToCreate = new Keypair();

  it('Create an account owned by our program', async () => {
    const instruction = SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
      space: 0,
      programId: program.programId, // Our program
    });

    const transaction = new Transaction();
    const blockhash = context.lastBlockhash;

    transaction.recentBlockhash = blockhash;
    transaction.add(instruction).sign(wallet.payer, accountToChange);
    await client.processTransaction(transaction);
  });

  it('Check accounts', async () => {
    await program.methods
      .checkAccounts()
      .accounts({
        payer: wallet.publicKey,
        accountToCreate: accountToCreate.publicKey,
        accountToChange: accountToChange.publicKey,
      })
      .rpc();
  });
});
