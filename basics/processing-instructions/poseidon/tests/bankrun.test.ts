import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { ProcessingInstructionsProgram } from '../target/types/processing_instructions_program';

const IDL = require('../target/idl/processing_instructions_program.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('custom-instruction-data', async () => {
  const context = await startAnchor('', [{ name: 'processing_instructions_program', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<ProcessingInstructionsProgram>(IDL, provider);

  it('It process an instruction!', async () => {
    // Anchor makes it super simple.
    await program.methods.processingInstructions(3).accounts({}).rpc();
    await program.methods.processingInstructions(10).accounts({}).rpc();
  });
});
