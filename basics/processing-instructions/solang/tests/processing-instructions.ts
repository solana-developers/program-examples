import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { ProcessingInstructions } from "../target/types/processing_instructions"

describe("processing-instructions", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  // Generate a new keypair for the data account for the program
  const dataAccount = anchor.web3.Keypair.generate()
  const wallet = provider.wallet

  const program = anchor.workspace
    .ProcessingInstructions as Program<ProcessingInstructions>

  it("Is initialized!", async () => {
    // Initialize data account for the program, which is required by Solang
    const tx = await program.methods
      .new(wallet.publicKey)
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount])
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Go to the park!", async () => {
    // Call the goToPark instruction on the program, providing the instruction data
    await program.methods
      .goToPark("Jimmy", 3)
      .accounts({ dataAccount: dataAccount.publicKey })
      .rpc()

    // Call the goToPark instruction on the program, providing the instruction data
    await program.methods
      .goToPark("Mary", 10)
      .accounts({ dataAccount: dataAccount.publicKey })
      .rpc()
  })
})
