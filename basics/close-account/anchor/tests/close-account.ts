import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import type { CloseAccountProgram } from '../target/types/close_account_program';

describe('close-an-account', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CloseAccountProgram as Program<CloseAccountProgram>;
  const payer = provider.wallet as anchor.Wallet;

  // Derive the PDA for the user's account.
  const [userAccountAddress] = PublicKey.findProgramAddressSync([Buffer.from('USER'), payer.publicKey.toBuffer()], program.programId);

  it('Create Account', async () => {
    await program.methods
      .createUser('John Doe')
      .accounts({
        user: payer.publicKey,
        userAccount: userAccountAddress,
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
        userAccount: userAccountAddress,
      })
      .rpc();

    // The account should no longer exist, returning null.
    const userAccount = await program.account.userState.fetchNullable(userAccountAddress);
    assert.equal(userAccount, null);
  });
});
