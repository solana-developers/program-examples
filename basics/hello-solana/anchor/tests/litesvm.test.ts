import assert from 'node:assert';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { HelloSolana } from '../target/types/hello_solana';

const IDL = require('../target/idl/hello_solana.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('anchor', () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<HelloSolana>;
  let payer: Keypair;

  before(async () => {
    // Configure the Anchor provider & load the program IDL for anchor-bankrun
    // The IDL gives you a typescript module
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new anchor.Program<HelloSolana>(IDL, provider);
  });

  it('Say hello!', async () => {
    const signature = await program.methods.hello().accounts({}).rpc();
    assert(signature);

    console.log(`Signature: ${signature}`);
  });
});
