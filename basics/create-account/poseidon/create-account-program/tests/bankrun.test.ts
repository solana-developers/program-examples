import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { getMint } from '@solana/spl-token';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import { CreateAccountProgram } from '../target/types/create_account_program';

const IDL = require('../target/idl/create_account_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Create a system account', async () => {
  const context = await startAnchor('', [{ name: 'create_account_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const program = anchor.workspace.CreateAccountProgram as anchor.Program<CreateAccountProgram>;

  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  it('Create the account', async () => {
    // Generate the public key from the seed and the programId
    const [accountState, _] = anchor.web3.PublicKey.findProgramAddressSync([anchor.utils.bytes.utf8.encode('account')], program.programId);
    console.log('wallet public key', wallet.publicKey);
    await program.methods
      .createSystemAccount()
      .accounts({
        owner: wallet.publicKey,
      })
      .signers([wallet.payer])
      .rpc();

    // Minimum balance for rent exemption for new account
    const lamports = await connection.getMinimumBalanceForRentExemption(0);

    // Check that the account was created
    // const accountInfo = await connection.getAccountInfo(accountState);
    // assert.isNotNull(accountInfo, 'Account should be created');
    // assert(accountInfo.lamports >= lamports, 'Account must have the minimum amount of lamports required for rent');
  });
});