import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { ProcessingInstructions } from '../target/types/processing_instructions';

const IDL = require('../target/idl/processing_instructions.json');

describe('anchor', () => {
  let client: any;
  let provider: LiteSVMProvider;
  let program: Program<ProcessingInstructions>;
  let payer: Keypair;

  before(async () => {
    // Configure the Anchor provider & load the program IDL for LiteSVM
    // The IDL gives you a typescript module
    client = fromWorkspace('');
    provider = new LiteSVMProvider(client);
    payer = provider.wallet.payer;
    program = new anchor.Program<ProcessingInstructions>(IDL, provider);
  });

  it('Go to the park!', async () => {
    // Anchor makes it super simple.
    await program.methods.goToPark('Jimmy', 3).accounts({}).rpc();
    await program.methods.goToPark('Mary', 10).accounts({}).rpc();
  });
});
