import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Rent } from "../target/types/rent"

describe("rent", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  // Generate a new keypair for the data account for the program
  const dataAccount = anchor.web3.Keypair.generate()
  const wallet = provider.wallet
  const connection = provider.connection

  const program = anchor.workspace.Rent as Program<Rent>

  it("Is initialized!", async () => {
    // Initialize data account for the program, which is required by Solang
    const tx = await program.methods
      .new(wallet.publicKey)
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount])
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Create new account", async () => {
    // Generate a new keypair for the new account
    const newAccount = anchor.web3.Keypair.generate()
    // Number of bytes of space to allocate for the account
    const space = 100
    // Get the minimum balance required for the account for the given space
    const lamports = await connection.getMinimumBalanceForRentExemption(space)

    // Create a new account via a Cross Program Invocation to the system program
    const tx = await program.methods
      .createSystemAccount(
        wallet.publicKey, // payer
        newAccount.publicKey, // new account
        new anchor.BN(lamports), // lamports
        new anchor.BN(space) // space
      )
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([newAccount]) // new account keypair required as signer
      .remainingAccounts([
        {
          pubkey: wallet.publicKey,
          isWritable: true,
          isSigner: true,
        },
        {
          pubkey: newAccount.publicKey, // new account
          isWritable: true,
          isSigner: true,
        },
      ])
      .rpc()
    console.log("Your transaction signature", tx)
  })
})
