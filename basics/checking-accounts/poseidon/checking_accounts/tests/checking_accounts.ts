import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { assert } from 'chai';
import type { CheckAccountsProgram } from '../target/types/check_accounts_program';

describe('Bankrun example', async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CheckAccountsProgram as anchor.Program<CheckAccountsProgram>;

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

    const transaction = new Transaction().add(instruction);

    await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer, accountToChange]);
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
