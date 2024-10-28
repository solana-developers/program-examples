import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RentExample } from "../target/types/rent_example";


describe("rent_example", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RentExample as Program<RentExample>;

  it("Checks if an account is rent-exempt", async () => {
      // Generate a new keypair for a user account
      const userAccount = anchor.web3.Keypair.generate();

      // Fund the user account with some lamports to cover rent (adjust as needed)
      await provider.connection.requestAirdrop(userAccount.publicKey, 1000000000);

      // Call the checkRentExemption function in our program
      const tx = await program.methods
          .checkRentExemption()
          .accounts({
              userAccount: userAccount.publicKey,
              rent: anchor.web3.SYSVAR_RENT_PUBKEY,          })
          .signers([userAccount])
          .rpc();

      console.log("Transaction signature", tx);
  });
});