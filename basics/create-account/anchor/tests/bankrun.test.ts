import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { Keypair, SystemProgram } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { CreateSystemAccount } from '../target/types/create_system_account';

const IDL = require('../target/idl/create_system_account.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Create a system account', async () => {
  const context = await startAnchor('', [{ name: 'create_system_account', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const wallet = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CreateSystemAccount>(IDL, provider);
  const connection = provider.connection;

  it('Create the account', async () => {
    // Generate a new keypair for the new account
    const newKeypair = new Keypair();

    await program.methods
      .createSystemAccount()
      .accounts({
        payer: wallet.publicKey,
        newAccount: newKeypair.publicKey,
      })
      .signers([newKeypair])
      .rpc();

    // Minimum balance for rent exemption for new account
    const lamports = await connection.getMinimumBalanceForRentExemption(0);

    // Check that the account was created
    const accountInfo = await connection.getAccountInfo(newKeypair.publicKey);
    assert(accountInfo.lamports === lamports);
  });
});
