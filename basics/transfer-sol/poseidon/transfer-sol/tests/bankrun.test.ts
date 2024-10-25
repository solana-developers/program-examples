import * as anchor from "@coral-xyz/anchor";
import { TransferSol } from "../target/types/transfer_sol";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";

const IDL = require("../target/idl/transfer_sol.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("Bankrun Transfer-SOL example", async () => {
  const context = await startAnchor(
    "",
    [{ name: "transfer_sol", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);
  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<TransferSol>(IDL, provider);

  const tranferAmount = 0.0001 * LAMPORTS_PER_SOL;

  const recipient = new Keypair();

  it("Transfer SOL", async () => {
    await printBalance(payer.publicKey, recipient.publicKey, "Before Transfer");

    await program.methods
      .transferSol(new anchor.BN(tranferAmount))
      .accounts({
        payer: payer.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();

    await printBalance(payer.publicKey, recipient.publicKey, "After Transfer");
  });

  async function printBalance(
    payer: PublicKey,
    recipient: PublicKey,
    when: string
  ) {
    const payerBalance = await provider.connection.getBalance(payer);
    const recipientBalance = await provider.connection.getBalance(recipient);
    console.log(
      `${when} - Payer: ${payerBalance / LAMPORTS_PER_SOL} - Recipient: ${
        recipientBalance / LAMPORTS_PER_SOL
      }`
    );
  }
});
