import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { getMint } from '@solana/spl-token';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import { CloseAccount } from '../target/types/close_account';

const IDL = require('../target/idl/close_account.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('close-account', async () => {
  // Configure the client to use the local cluster.
  const context = await startAnchor('', [{ name: 'close_account_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const program = anchor.workspace.CloseAccount as anchor.Program<CloseAccount>;

  const user = provider.wallet as anchor.Wallet;

  // Variables that will store the user account PDA and its bump
  let userAccount: PublicKey;
  let userAccountBump: number;

  it('Create User Account', async () => {
    [userAccount, userAccountBump] = await PublicKey.findProgramAddressSync([Buffer.from('user'), user.publicKey.toBuffer()], program.programId);

    // Create User Account instruction invoked from the program
    await program.methods
      .createUser()
      .accounts({
        user: user.publicKey, // User's public key
      })
      .signers([user.payer]) // Sign the transaction with the user's keypair
      .rpc();

    // Fetch and assert the accounts data
    const userAccountData = await program.account.closeAccountState.fetch(userAccount);

    console.log('Created User Account:', userAccount.toBase58());
    console.log('Account user Public Key:', userAccountData.user.toBase58());

    assert.equal(userAccountData.user.toBase58(), user.publicKey.toBase58()); // Verify the user account data
  });

  it('Close User Account', async () => {
    // Close User Account instruction invoked from the program
    await program.methods
      .closeUser()
      .accounts({
        user: user.publicKey, // User's public key
      })
      .signers([user.payer]) // Sign the transaction with the user's keypair
      .rpc();

    // The account should no longer exist, returning null.
    try {
      const userAccountData = await program.account.closeAccountState.fetchNullable(userAccount);
      assert.equal(userAccountData, null); // Verify the user account is closed
    } catch (err) {
      // Won't return null and will throw an error in anchor-bankrun'
      assert.equal(err.message, `Could not find ${userAccount}`);
    }
  });
});
