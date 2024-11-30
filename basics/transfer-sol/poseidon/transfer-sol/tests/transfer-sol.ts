import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferSol } from "../target/types/transfer_sol";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

describe("transfer-sol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TransferSol as Program<TransferSol>;

  it("Is SOL transferred!", async () => {
    const payer = provider.wallet;
    const recipient = anchor.web3.Keypair.generate();
    const transferAmount = new anchor.BN(1000000);

    await printBalance(payer.publicKey, recipient.publicKey, "Before Transfer");

    const tx = await program.methods
      .transferSol(transferAmount)
      .accounts({
        payer: payer.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();

    await printBalance(payer.publicKey, recipient.publicKey, "After Transfer");

    console.log("Your transaction signature", tx);
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
