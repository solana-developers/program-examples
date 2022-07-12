import * as anchor from "@project-serum/anchor";
import { CustomInstructionData } from "../target/types/custom_instruction_data";


describe("custom-instruction-data", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CustomInstructionData as anchor.Program<CustomInstructionData>;

  it("Go to the park!", async () => {
    
    // Again, Anchor makes it super simple.
    //
    await program.methods.goToPark("Jimmy", 3)
    .accounts({})
    .rpc();
    await program.methods.goToPark("Mary", 10)
    .accounts({})
    .rpc();

  });
});
