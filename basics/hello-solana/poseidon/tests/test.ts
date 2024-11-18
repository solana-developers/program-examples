import * as anchor from "@coral-xyz/anchor";
import { HelloSolanaProgram } from "../target/types/hello_solana_program";

describe("hello-solana", () => {
  // Configure the Anchor provider & load the program IDL
  // The IDL gives you a typescript module
  //
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace
    .HelloSolanaProgram as anchor.Program<HelloSolanaProgram>;

  it("Say hello!", async () => {
    await program.methods.helloSolana().accounts({}).rpc();
  });
});
