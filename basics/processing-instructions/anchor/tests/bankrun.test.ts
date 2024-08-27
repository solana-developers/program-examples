import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { ProcessingInstructions } from '../target/types/processing_instructions';

const IDL = require('../target/idl/processing_instructions.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('custom-instruction-data', async () => {
  const context = await startAnchor('', [{ name: 'processing_instructions', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<ProcessingInstructions>(IDL, provider);

  it('Go to the park!', async () => {
    // Anchor makes it super simple.
    await program.methods.goToPark('Jimmy', 3).accounts({}).rpc();
    await program.methods.goToPark('Mary', 10).accounts({}).rpc();
  });
});
