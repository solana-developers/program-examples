import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { AnchorRealloc } from '../target/types/anchor_realloc';

const IDL = require('../target/idl/anchor_realloc.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('anchor-realloc', async () => {
  const context = await startAnchor('', [{ name: 'anchor_realloc', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const connection = provider.connection;
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<AnchorRealloc>(IDL, provider);

  const messageAccount = new Keypair();

  // helper function to check the account data and message
  async function checkAccount(publicKey: PublicKey, expectedMessage: string) {
    const accountInfo = await connection.getAccountInfo(publicKey);
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
