import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";

describe("hello-solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Generate a new random keypair for the data account.
  const dataAccount = anchor.web3.Keypair.generate();
  const wallet = provider.wallet;

  const program = anchor.workspace.HelloSolana as Program<HelloSolana>;

  it("Is initialized!", async () => {
    // Initialize a new data account
    const tx = await program.methods
      .new() // wallet.publicKey is the payer for the new account
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount]) // dataAccount keypair is a required signer because we're using it to create a new account
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
