import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProcessingInstructionsProgram } from "../target/types/processing_instructions_program";

describe("processing-instructions", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ProcessingInstructions as Program<ProcessingInstructionsProgram>;

  it("It process an instruction!", async () => {
    // Add your test here.
    await program.methods.processingInstructions(3).accounts({}).rpc();
    await program.methods.processingInstructions(10).accounts({}).rpc();
  });
});

