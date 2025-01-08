import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { startAnchor } from 'solana-bankrun';
import type { ProcessingInstructionsProgram } from '../target/types/processing_instructions_program';

const IDL = require('../target/idl/processing_instructions_program.json');
const PROGRAM_ID = new anchor.web3.PublicKey(IDL.address);

describe('Bankrun - processing_instructions_program', async () => {
  // Start the Bankrun context
  const context = await startAnchor('', [{ name: 'processing_instructions_program', programId: PROGRAM_ID }], []);

  const provider = new BankrunProvider(context);
  anchor.setProvider(provider);
  const program = new anchor.Program<ProcessingInstructionsProgram>(IDL, provider);

  const payer = provider.wallet as anchor.Wallet;

  it('Tests the go_to_park function', async () => {
    // Define the test parameters
    const height = 6;
    const name = 'Alice';

    // Call the go_to_park function in the Solana program
    const tx = await program.methods
      .goToPark(height, name)
      .accounts({
        user: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log('Your transaction signature', tx);

    // Assertions can be made here based on expected behavior
    // Since we are using msg! for console messages, we don't have a direct way to capture these outputs in tests
    // However, we can still verify the transaction was successful
    assert.isNotNull(tx, 'Transaction should be successful');
  });
});
