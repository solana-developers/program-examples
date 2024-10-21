import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { ProcessingInstructionsPoseidon } from '../target/types/processing_instructions_poseidon';

const IDL = require('../target/idl/processing_instructions_poseidon.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('processing-instructions-poseidon', async () => {
  const context = await startAnchor('', [{ name: 'processing_instructions_poseidon', programId: PROGRAM_ID }], []);
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<ProcessingInstructionsPoseidon>(IDL, provider);

  it('Go to the park with Poseidon!', async () => {
    await program.methods.goToPark('Jimmy', 3).accounts({}).rpc();
    await program.methods.goToPark('Mary', 10).accounts({}).rpc();
  });
});
