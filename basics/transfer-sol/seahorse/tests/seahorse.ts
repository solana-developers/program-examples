import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Seahorse } from "../target/types/seahorse";

describe("seahorse", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Seahorse as Program<Seahorse>;

  const mockAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mock_account")],
    program.programId
  );

  it("Initialize the Mock account to send our SOL to", async () => {
    const tx = await program.methods
      .initMockAccount()
      .accounts({
        mockAccount: mockAccount[0],
        signer: program.provider.publicKey,
      })
      .rpc();
  });
  it("Send SOL To Mock account", async () => {
    const amount = 1;
    // Convert to lamport.
    const lamports: number = anchor.web3.LAMPORTS_PER_SOL * amount;
    const tx = await program.methods
      .transfer(new anchor.BN(lamports))
      .accounts({
        recipient: mockAccount[0],
        sender: program.provider.publicKey,
      })
      .rpc();
    console.log("Your transaction signature: ", tx);
  });
});
