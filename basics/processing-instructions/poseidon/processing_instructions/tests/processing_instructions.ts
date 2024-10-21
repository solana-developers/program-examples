import { Idl, Program } from '@coral-xyz/anchor';
import { BankrunProvider } from 'anchor-bankrun';
import { ProgramTestContext, startAnchor } from 'solana-bankrun';
import ProcessingInstructionsIDL from '../target/idl/processing_instructions.json';
import { ProcessingInstructions } from '../target/types/processing_instructions';

describe('processing_instructions', () => {
  let context: ProgramTestContext;
  let provider: BankrunProvider;
  let processingInstructionsProgram: Program<ProcessingInstructions>;

  before(async () => {
    context = await startAnchor('../processing_instructions', [], []);
    provider = new BankrunProvider(context);
    processingInstructionsProgram = new Program(ProcessingInstructionsIDL as Idl, provider) as unknown as Program<ProcessingInstructions>;
  });

  it('Go to the park!', async () => {
    await processingInstructionsProgram.methods.goToPark('Jimmy').accounts({}).rpc();
    await processingInstructionsProgram.methods.goToPark('Mary').accounts({}).rpc();
  });
});
