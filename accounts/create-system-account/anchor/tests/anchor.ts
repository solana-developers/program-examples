import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CreateSystemAccount } from "../target/types/create_system_account";

describe("anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Anchor as Program<CreateSystemAccount>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
