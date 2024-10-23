import * as anchor from '@coral-xyz/anchor';
import type { ProcessingInstructionsPoseidon } from '../target/types/processing_instructions_poseidon';

describe('processing-instructions-poseidon', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ProcessingInstructionsPoseidon as anchor.Program<ProcessingInstructionsPoseidon>;

  it('Go to the park with Poseidon!', async () => {
    await program.methods.goToPark('Jimmy', 3).accounts({}).rpc();
    await program.methods.goToPark('Mary', 10).accounts({}).rpc();
  });
});
