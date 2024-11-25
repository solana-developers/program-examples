import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BN } from "bn.js";
import { TransferSol } from "../target/types/transfer_sol";

describe("transfer_sol", () => {
  //UTILS
  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    console.log(block.blockhash);
    console.log(block.lastValidBlockHeight);
    const res = await provider.connection.confirmTransaction({
      signature,
      ...block,
    });
    console.log(res);

    return signature;
  };

  //SET ENVIRONMENT
  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.TransferSol as Program<TransferSol>;
  anchor.setProvider(provider);
  const maker = anchor.web3.Keypair.generate();
  const taker = anchor.web3.Keypair.generate();
  const airdropAmount = 2.5 * anchor.web3.LAMPORTS_PER_SOL;
  const transferAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;

  it("Airdrop", async () => {
    const airdropMaker = await provider.connection
      .requestAirdrop(maker.publicKey, airdropAmount)
      .then(confirm);
    console.log("\nAirdropped 5 SOL to maker", airdropMaker);
  });

  it("Transfer token", async () => {
    // Add your test here.
    const tx = await program.methods
      .transferWithProgram(new BN(transferAmount))
      .accounts({
        from: maker.publicKey,
        to: taker.publicKey,
      })
      .signers([maker])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
