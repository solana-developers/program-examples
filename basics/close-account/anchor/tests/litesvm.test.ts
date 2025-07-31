import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { assert } from 'chai';
import { CloseAccountProgram } from '../target/types/close_account_program';
const IDL = require('../target/idl/close_account_program.json');

describe('close-an-account', async () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<CloseAccountProgram>;
  let payer: Keypair;
  let userAccountAddress: PublicKey;

  before(async () => {
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new Program<CloseAccountProgram>(IDL, provider);

    // Derive the PDA for the user's account.
    [userAccountAddress] = PublicKey.findProgramAddressSync([Buffer.from('USER'), payer.publicKey.toBuffer()], program.programId);
  });

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
