import * as anchor from "@project-serum/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";

describe("Anchor example", () => {
  
  // Configure the Anchor provider & load the program IDL
  // The IDL gives you a typescript module
  //
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AnchorProgramExample as anchor.Program<AnchorProgramExample>;

  it("Test our example", async () => {
    
    // Just run Anchor's IDL method to build a transaction!
    //
    await program.methods.hello()
    .accounts({})
    .rpc();

  });
});
