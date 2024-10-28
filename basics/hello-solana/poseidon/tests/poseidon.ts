import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";

describe("poseidon", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Poseidon as Program<HelloSolana>;

  it("Say hello!", async () => {
    const tx = await program.methods.hello().rpc();
    console.log("Your transaction signature", tx);
  });
});
