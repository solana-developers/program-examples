import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferSol } from "../target/types/transfer_sol";

describe("transfer-sol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TransferSol as Program<TransferSol>;

  it("Is SOL transferred!", async () => {
    // Add your test here.
    const tx = await program.methods
      .transferSol(new anchor.BN(1000000))
      .accounts({
        payer: provider.wallet.publicKey,
        recipient: anchor.web3.Keypair.generate().publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
