import assert from 'node:assert';
import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { CloseAccountProgram } from '../target/types/close_account_program';

const IDL = require('../target/idl/close_account_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('close-an-account', async () => {
  // Configure the client to use the local cluster.
  const context = await startAnchor('', [{ name: 'close_account_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CloseAccountProgram>(IDL, provider);
  // Derive the PDA for the user's account.
  const [userAccountAddress] = PublicKey.findProgramAddressSync([Buffer.from('USER'), payer.publicKey.toBuffer()], program.programId);

  it('Create Account', async () => {
    await program.methods
      .createUser('John Doe')
      .accounts({
        user: payer.publicKey,
      })
      .rpc();

    // Fetch the account data
    const userAccount = await program.account.userState.fetch(userAccountAddress);
    assert.equal(userAccount.name, 'John Doe');
    assert.equal(userAccount.user.toBase58(), payer.publicKey.toBase58());
  });

  it('Close Account', async () => {
    await program.methods
      .closeUser()
      .accounts({
        user: payer.publicKey,
      })
      .rpc();

    // The account should no longer exist, returning null.
    try {
      const userAccount = await program.account.userState.fetchNullable(userAccountAddress);
      assert.equal(userAccount, null);
    } catch (err) {
      // Won't return null and will throw an error in anchor-bankrun'
      assert.equal(err.message, `Could not find ${userAccountAddress}`);
    }
  });
});
