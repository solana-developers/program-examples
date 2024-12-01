import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferSol } from "../target/types/transfer_sol";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
describe("transfer-sol", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TransferSol as Program<TransferSol>;
  // Generate new user keypairs for testing
  const user = Keypair.generate();
  const receiver = Keypair.generate();
  // Set the transfer amount to 1 SOL
  const transferAmount = 1 * LAMPORTS_PER_SOL;
  before(async () => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    // Airdrop 5 SOL to the user that will send SOL to the other user
    const airdropUser = await provider.connection.requestAirdrop(
      user.publicKey,
      5 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropUser,
    });
  });
  it("Transfer SOL with CPI", async () => {
    await getBalances(user.publicKey, receiver.publicKey, "\n Beginning");
    // Transfer SOL instruction invoked from the program
    await program.methods
      .transferSolWithCpi(new anchor.BN(transferAmount))
      .accountsPartial({
        payer: user.publicKey,
        recipient: receiver.publicKey,
      })
      .signers([user])
      .rpc();
    await getBalances(user.publicKey, receiver.publicKey, "\n Resulting");
  });
  // Helper function to display balance of the accounts
  async function getBalances(
    payerPubkey: PublicKey,
    recipientPubkey: PublicKey,
    timeframe: string
  ) {
    const payerBalance = await provider.connection.getBalance(payerPubkey);
    const recipientBalance = await provider.connection.getBalance(
      recipientPubkey
    );
    console.log(`${timeframe} balances:`);
    console.log(`   Payer: ${payerBalance / LAMPORTS_PER_SOL}`);
    console.log(`   Recipient: ${recipientBalance / LAMPORTS_PER_SOL}`);
  }
});
