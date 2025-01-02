import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { HelloSolanaProgram } from "../target/types/hello_solana_program"; // Assuming this path

describe("hello_solana_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .HelloSolanaProgram as Program<HelloSolanaProgram>;

  it("Executes 'hello' successfully", async () => {
    const tx = await program.methods.hello().rpc();

    // Chai assert to ensure no error occurred and transaction completed successfully
    assert.isOk(tx, "Transaction should complete without errors");
  });
});
