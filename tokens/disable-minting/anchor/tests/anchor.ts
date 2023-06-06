import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Anchor } from "../target/types/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Anchor as Program<Anchor>;
  const connection = program.provider.connection;

  const payer = anchor.web3.Keypair.generate();

  const mintPair = anchor.web3.Keypair.generate();
  const mint = mintPair.publicKey;

  it("Create Token and freeze mint authority", async () => {
    const tx = new anchor.web3.Transaction();

    const airdropSignature = await connection.requestAirdrop(
      payer.publicKey,
      1 * 10 ** 9
    );

    await connection.confirmTransaction(airdropSignature);

    const sig = await program.methods
      .mintAndDisableMint()
      .accounts({
        mintAccount: mint,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([payer, mintPair])
      .rpc();

    console.log("Your transaction signature: ", sig);
  });
});
