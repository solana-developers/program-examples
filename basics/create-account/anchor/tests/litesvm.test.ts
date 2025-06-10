import assert from 'node:assert';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { CreateSystemAccount } from '../target/types/create_system_account';
const IDL = require('../target/idl/create_system_account.json');

describe('anchor', () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<CreateSystemAccount>;
  let payer: Keypair;
  let connection: LiteSVMProvider['connection'];

  before(async () => {
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new Program<CreateSystemAccount>(IDL, provider);
    connection = provider.connection;
  });

  it('Create the account', async () => {
    // Generate a new keypair for the new account
    const newKeypair = new Keypair();

    await program.methods
      .createSystemAccount()
      .accounts({
        payer: payer.publicKey,
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
