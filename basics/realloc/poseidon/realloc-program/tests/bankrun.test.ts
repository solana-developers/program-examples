import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { ReallocProgram } from '../target/types/realloc_program';

const IDL = require('../target/idl/realloc_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('realloc_program', async () => {
  const context = await startAnchor('', [{ name: 'realloc_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  const connection = provider.connection;
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<ReallocProgram>(IDL, provider);

  // Define the message account
  const messageAccount = Keypair.generate();
  let messagePDA: PublicKey;
  let bump: number;

  // Helper function to check account data and message
  async function checkAccount(publicKey: PublicKey, expectedMessage: string) {
    const accountData = await program.account.messageAccountState.fetch(publicKey);

    // Verify the message and bump
    assert.equal(accountData.message, expectedMessage, 'Message should match expected value');
    assert.equal(accountData.bump, bump, 'Bump should match expected value');
  }

  it('initialize the message account', async () => {
    const initialMessage = 'Hello, Solana!';

    // Call the initialize instruction
    await program.methods
      .initialize(initialMessage)
      .accounts({
        payer: payer.publicKey,
      })
      .signers([])
      .rpc();

    [messagePDA, bump] = await PublicKey.findProgramAddress([Buffer.from('message')], program.programId);

    // Verify the account data
    await checkAccount(messagePDA, initialMessage);
  });

  it('update the message account', async () => {
    const updatedMessage = 'Updated Message';

    // Call the update instruction
    await program.methods
      .update(updatedMessage)
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();

    // Verify the account data
    await checkAccount(messagePDA, updatedMessage);
  });
});
