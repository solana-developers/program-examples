import * as anchor from "@coral-xyz/anchor"
import { TransferSol } from "../target/types/transfer_sol"
import {
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js"
describe("transfer-sol", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet
  const program = anchor.workspace.TransferSol as anchor.Program<TransferSol>

  const transferAmount = 1 * LAMPORTS_PER_SOL

  // Generate a new keypair for the recipient
  const recipient = new Keypair()

  // Generate a new keypair to create an account owned by our program
  const programOwnedAccount = new Keypair()

  it("Transfer SOL with CPI", async () => {
    await getBalances(payer.publicKey, recipient.publicKey, "Beginning")

    await program.methods
      .transferSolWithCpi(new anchor.BN(transferAmount))
      .accounts({
        payer: payer.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc()

    await getBalances(payer.publicKey, recipient.publicKey, "Resulting")
  })

  it("Create and fund account owned by our program", async () => {
    const instruction = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: programOwnedAccount.publicKey,
      space: 0,
      lamports: 1 * LAMPORTS_PER_SOL, // 1 SOL
      programId: program.programId, // Program Owner, our program's address
    })

    const transaction = new Transaction().add(instruction)

    await sendAndConfirmTransaction(provider.connection, transaction, [
      payer.payer,
      programOwnedAccount,
    ])
  })

  it("Transfer SOL with Program", async () => {
    await getBalances(
      programOwnedAccount.publicKey,
      payer.publicKey,
      "Beginning"
    )

    await program.methods
      .transferSolWithProgram(new anchor.BN(transferAmount))
      .accounts({
        payer: programOwnedAccount.publicKey,
        recipient: payer.publicKey,
      })
      .rpc()

    await getBalances(
      programOwnedAccount.publicKey,
      payer.publicKey,
      "Resulting"
    )
  })

  async function getBalances(
    payerPubkey: PublicKey,
    recipientPubkey: PublicKey,
    timeframe: string
  ) {
    let payerBalance = await provider.connection.getBalance(payerPubkey)
    let recipientBalance = await provider.connection.getBalance(recipientPubkey)
    console.log(`${timeframe} balances:`)
    console.log(`   Payer: ${payerBalance / LAMPORTS_PER_SOL}`)
    console.log(`   Recipient: ${recipientBalance / LAMPORTS_PER_SOL}`)
  }
})
