import * as anchor from "@coral-xyz/anchor"
import { Hand } from "../target/types/hand"
import { Lever } from "../target/types/lever"
import { Keypair } from "@solana/web3.js"

describe("CPI Example", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const hand = anchor.workspace.Hand as anchor.Program<Hand>
  const lever = anchor.workspace.Lever as anchor.Program<Lever>

  // Generate a new keypair for the power account
  const powerAccount = new Keypair()

  it("Initialize the lever!", async () => {
    await lever.methods
      .initialize()
      .accounts({
        power: powerAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([powerAccount])
      .rpc()
  })

  it("Pull the lever!", async () => {
    await hand.methods
      .pullLever("Chris")
      .accounts({
        power: powerAccount.publicKey,
        leverProgram: lever.programId,
      })
      .rpc()
  })

  it("Pull it again!", async () => {
    await hand.methods
      .pullLever("Ashley")
      .accounts({
        power: powerAccount.publicKey,
        leverProgram: lever.programId,
      })
      .rpc()
  })
})
