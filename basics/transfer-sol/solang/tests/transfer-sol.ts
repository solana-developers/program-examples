import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferSol } from "../target/types/transfer_sol";
import { PublicKey } from "@solana/web3.js";

describe("transfer-sol", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Generate new keypair to use as data account
  const dataAccount = anchor.web3.Keypair.generate();
  const wallet = provider.wallet;
  const connection = provider.connection;

  const program = anchor.workspace.TransferSol as Program<TransferSol>;

  // Amount to transfer in lamports
  const transferAmount = 1 * anchor.web3.LAMPORTS_PER_SOL; // 1 SOL

  it("Is initialized!", async () => {
    // Create the new data account
    // The dataAccount is required by Solang even though it is not use in the transfer instruction
    const tx = await program.methods
      .new()
      .accounts({ dataAccount: dataAccount.publicKey })
      .signers([dataAccount])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Transfer SOL using CPI to the system program", async () => {
    // Generate new keypair to use as recipient for the transfer
    const recipient = anchor.web3.Keypair.generate(); // test1 recipient

    await getBalances(wallet.publicKey, recipient.publicKey, "Beginning");

    const tx = await program.methods
      .transferSolWithCpi(
        new anchor.BN(transferAmount) // amount in lamports
      )
      .accounts({
        sender: wallet.publicKey,
        recipient: recipient.publicKey
       })
      .rpc();

    await getBalances(wallet.publicKey, recipient.publicKey, "Resulting");

    console.log("Your transaction signature", tx);
  });

  it("Transfer SOL to program owned account", async () => {
    await getBalances(wallet.publicKey, dataAccount.publicKey, "Beginning");

    const tx = await program.methods
      .transferSolWithCpi(
        new anchor.BN(transferAmount) // amount in lamports
      )
      .accounts({ 
        sender: wallet.publicKey,
        recipient: dataAccount.publicKey,
       })
      .rpc();

    await getBalances(wallet.publicKey, dataAccount.publicKey, "Resulting");

    console.log("Your transaction signature", tx);
  });

  it("Transfer SOL from program owned account", async () => {
    await getBalances(dataAccount.publicKey, wallet.publicKey, "Beginning");

    const tx = await program.methods
      .transferSolWithProgram(
        new anchor.BN(transferAmount) // amount in lamports
      )
      .accounts({ 
        sender: dataAccount.publicKey,
        recipient: wallet.publicKey,
       })
      .rpc();

    await getBalances(dataAccount.publicKey, wallet.publicKey, "Resulting");

    console.log("Your transaction signature", tx);
  });

  // Helper function to get balances and log them to the console
  async function getBalances(
    payerPubkey: PublicKey,
    recipientPubkey: PublicKey,
    timeframe: string
  ) {
    let payerBalance = await connection.getBalance(payerPubkey);
    let recipientBalance = await connection.getBalance(recipientPubkey);
    console.log(`${timeframe} balances:`);
    console.log(`   Payer: ${payerBalance / anchor.web3.LAMPORTS_PER_SOL}`); // convert lamports to SOL
    console.log(
      `   Recipient: ${recipientBalance / anchor.web3.LAMPORTS_PER_SOL}` // convert lamports to SOL
    );
  }
});
