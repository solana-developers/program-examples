import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProgramDerivedAddressesSeahorse } from "../target/types/program_derived_addresses_seahorse";

describe("program_derived_addresses_seahorse", () => {
  // Set the provider to use the local environment
  anchor.setProvider(anchor.AnchorProvider.env());

  // Get the program instance from the workspace
  const program = anchor.workspace
    .ProgramDerivedAddressesSeahorse as Program<ProgramDerivedAddressesSeahorse>;

  it("Is Created!", async () => {
    try {
      // Call the createPageVisits method on the program
      const tx = await program.methods.createPageVisits().rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      // Handle the case where the account is already created
      if (error.message.includes("already in use")) {
        console.log("Account already created, skipping creation.");
      } else {
        throw error;
      }
    }
  });

  it("Is Updated!", async () => {
    // Get the public key of the wallet owner
    const owner = anchor.AnchorProvider.env().wallet.publicKey;

    // Derive the pageVisitAccount address using the program ID and a seed
    const [pageVisitAccount] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("page_visits"), owner.toBuffer()],
      program.programId
    );

    // Call the incrementPageVisits method on the program
    const tx = await program.methods
      .incrementPageVisits()
      .accounts({
        pageVisits: pageVisitAccount,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    // Fetch and log the value of pageVisits
    const account = await program.account.pageVisits.fetch(pageVisitAccount);
    console.log("Page Visits:", account);
  });
});
