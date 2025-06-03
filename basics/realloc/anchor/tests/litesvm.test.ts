import assert from 'node:assert';
import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { AnchorRealloc } from '../target/types/anchor_realloc';

const IDL = require('../target/idl/anchor_realloc.json');

describe('anchor', () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<AnchorRealloc>;
  let payer: Keypair;

  const messageAccount = new Keypair();

  before(async () => {
    // Configure the Anchor provider & load the program IDL for LiteSVM
    // The IDL gives you a typescript module
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new anchor.Program<AnchorRealloc>(IDL, provider);
  });

  // helper function to check the account data and message
  async function checkAccount(publicKey: PublicKey, expectedMessage: string) {
    const accountInfo = await provider.connection.getAccountInfo(publicKey);
    const accountData = await program.account.message.fetch(publicKey);

    // 8 bytes for the discriminator,
    // 4 bytes for the length of the message,
    // and the length of the message
    assert.equal(accountInfo.data.length, 8 + 4 + expectedMessage.length);
    assert.equal(accountData.message, expectedMessage);

    console.log(`Account Data Length: ${accountInfo.data.length}`);
    console.log(`Message: ${accountData.message}`);
  }

  it('Is initialized!', async () => {
    const input = 'hello';

    await program.methods
      .initialize(input)
      .accounts({
        payer: payer.publicKey,
        messageAccount: messageAccount.publicKey,
      })
      .signers([messageAccount])
      .rpc();

    await checkAccount(messageAccount.publicKey, input);
  });

  it('Update', async () => {
    const input = 'hello world';

    await program.methods
      .update(input)
      .accounts({
        payer: payer.publicKey,
        messageAccount: messageAccount.publicKey,
      })
      .rpc();

    await checkAccount(messageAccount.publicKey, input);
  });

  it('Update', async () => {
    const input = 'hi';

    await program.methods
      .update(input)
      .accounts({
        payer: payer.publicKey,
        messageAccount: messageAccount.publicKey,
      })
      .rpc();

    await checkAccount(messageAccount.publicKey, input);
  });
});
