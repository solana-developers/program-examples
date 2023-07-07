import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { ProgramDerivedAddresses } from "../target/types/program_derived_addresses"
import { PublicKey } from "@solana/web3.js"

describe("program-derived-addresses", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const wallet = provider.wallet

  const program = anchor.workspace
    .ProgramDerivedAddresses as Program<ProgramDerivedAddresses>

  // Derive the PDA that will be used to initialize the dataAccount.
  const [dataAccountPDA, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("page_visits"), wallet.publicKey.toBuffer()],
    program.programId
  )

  it("Is initialized!", async () => {
    // Initialize the dataAccount using a PDA as the address.
    // The PDA doesn't have to be passed in explicity as a signer, the program can "sign" for it.
    // This is a feature of PDAs that allows programs to "sign" for PDA that are derived from their programId.
    const tx = await program.methods
      .new(
        wallet.publicKey, // wallet.publicKey is the payer for the new account
        [bump] // bump seed for the PDA found using findProgramAddress
      )
      .accounts({ dataAccount: dataAccountPDA })
      .rpc({ skipPreflight: true })
    console.log("Your transaction signature", tx)

    // Get the current state of the dataAccount.
    const val = await program.methods
      .get()
      .accounts({ dataAccount: dataAccountPDA })
      .view()

    console.log("State:", val)
  })

  it("Visit the page!", async () => {
    // Increment the page visits counter.
    const tx = await program.methods
      .incrementPageVisits()
      .accounts({ dataAccount: dataAccountPDA })
      .rpc()
    console.log("Your transaction signature", tx)

    // Get the current state of the dataAccount.
    const val = await program.methods
      .get()
      .accounts({ dataAccount: dataAccountPDA })
      .view()

    console.log("State:", val)
  })
})
