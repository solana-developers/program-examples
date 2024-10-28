import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import { ProcessingInstructionsProgram } from '../target/types/processing_instructions_program';

describe('processing_instructions_program', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ProcessingInstructionsProgram as Program<ProcessingInstructionsProgram>;
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
