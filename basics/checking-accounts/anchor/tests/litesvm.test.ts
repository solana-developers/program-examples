import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey, SystemProgram, Transaction } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { AnchorProgramExample } from '../target/types/anchor_program_example';
const IDL = require('../target/idl/anchor_program_example.json');

describe('anchor', () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<AnchorProgramExample>;
  let payer: Keypair;
  let accountToChange: Keypair;
  let accountToCreate: Keypair;

  before(async () => {
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new Program<AnchorProgramExample>(IDL, provider);

    // We'll create this ahead of time.
    // Our program will try to modify it.
    accountToChange = new Keypair();
    // Our program will create this.
    accountToCreate = new Keypair();
  });

  it('Create an account owned by our program', async () => {
    const instruction = SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
      space: 0,
      programId: program.programId, // Our program
    });

    const transaction = new Transaction();
    const blockhash = provider.client.latestBlockhash();

    transaction.recentBlockhash = blockhash;
    transaction.add(instruction).sign(payer, accountToChange);
    await provider.sendAndConfirm(transaction);
  });

  it('Check accounts', async () => {
    await program.methods
      .checkAccounts()
      .accounts({
        payer: payer.publicKey,
        accountToCreate: accountToCreate.publicKey,
        accountToChange: accountToChange.publicKey,
      })
      .rpc();
  });
});
