import * as anchor from "@project-serum/anchor";
import { TransferSol } from "../target/types/transfer_sol";

describe("transfer-sol", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.TransferSol as anchor.Program<TransferSol>;

  it("Transfer some SOL", async () => {

    async function getBalances(payerPubkey: anchor.web3.PublicKey, recipientPubkey: anchor.web3.PublicKey, timeframe: string) {
      let payerBalance = await provider.connection.getBalance(payerPubkey);
      let recipientBalance = await provider.connection.getBalance(recipientPubkey);
      console.log(`${timeframe} balances:`);
      console.log(`   Payer: ${payerBalance}`);
      console.log(`   Recipient: ${recipientBalance}`);
    };

    const recipientKeypair = anchor.web3.Keypair.generate();
    const transferAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;

    await getBalances(wallet.publicKey, recipientKeypair.publicKey, "Beginning");
    
    await program.methods.transferSol(new anchor.BN(transferAmount))
    .accounts({
      payer: wallet.publicKey,
      recipient: recipientKeypair.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([wallet.payer])
    .rpc();

    await getBalances(wallet.publicKey, recipientKeypair.publicKey, "Resulting");

  });
});
