import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { CheckAccountsProgram } from '../target/types/check_accounts_program';

const IDL = require('../target/idl/check_accounts_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Bankrun example', async () => {
  const context = await startAnchor('', [{ name: 'checking_accounts', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CheckAccountsProgram>(IDL, provider);
  const client = context.banksClient;

  // We'll create this ahead of time.
  // Our program will try to modify it.
  const accountToChange = Keypair.generate();
  // Our program will create this.
  const accountToCreate = Keypair.generate();

  it('Create an account owned by our program', async () => {
    const instruction = SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: accountToCreate.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
      space: 0,
      programId: program.programId, // Our program
    });

    const transaction = new Transaction();
    const blockhash = context.lastBlockhash;

    transaction.recentBlockhash = blockhash;
    transaction.add(instruction).sign(wallet.payer, accountToCreate);
    await client.processTransaction(transaction);
    // Fetch account info to ensure it's created correctly
    const accountInfo = await provider.connection.getAccountInfo(accountToCreate.publicKey);
    assert.isNotNull(accountInfo, 'Account was not created');
    assert.strictEqual(accountInfo.owner.toBase58(), program.programId.toBase58(), 'Account is not owned by the program');
  });

  //Checks accounts based on constraints defined in the checkAccounts method
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
