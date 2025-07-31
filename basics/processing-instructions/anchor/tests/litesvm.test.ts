import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { LiteSVMProvider, fromWorkspace } from 'anchor-litesvm';
import { ProcessingInstructions } from '../target/types/processing_instructions';

const IDL = require('../target/idl/processing_instructions.json');

it('Go to the park!', async () => {
  const client = fromWorkspace('');
  const provider = new LiteSVMProvider(client);
  const payer = provider.wallet.payer;
  const program = new anchor.Program<ProcessingInstructions>(IDL, provider);

  await program.methods.goToPark('Jimmy', 3).accounts({}).rpc();
  await program.methods.goToPark('Mary', 10).accounts({}).rpc();
});
