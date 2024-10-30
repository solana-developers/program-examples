import assert from "node:assert";
import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import type { CloseAccount } from "../target/types/close_account";

  const connection = new Connection("https://api.devnet.solana.com");

  async function createAndProcessTransaction(
    payer: Keypair,
    instruction: TransactionInstruction,
    additionalSigners: Keypair[] = []
  ): Promise<{ transaction: VersionedTransaction; signature: string | null }> {
    // Get the latest blockhash
    const { blockhash, lastValidBlockHeight } = await getLatestBlockhash();
    const message = new TransactionMessage({
        payerKey: payer.publicKey,
        recentBlockhash: blockhash,
        instructions: [instruction],
    }).compileToV0Message();

    const tx = new VersionedTransaction(message);

    try {
      const signature = await sendAndConfirmTransaction(tx);
      return { transaction: tx, signature };
    } catch (err) {
      return { transaction: tx, signature: null };
    }
  }

  async function getLatestBlockhash(): Promise<{
    blockhash: string;
    lastValidBlockHeight: number;
  }> {
    const { blockhash, lastValidBlockHeight } =
      await connection.getLatestBlockhash("finalized");
    return { blockhash, lastValidBlockHeight };
  }

  async function sendAndConfirmTransaction(tx: VersionedTransaction): Promise<string> {

    const signature = await connection.sendTransaction(tx);

    const { blockhash, lastValidBlockHeight } = await getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: blockhash,
      lastValidBlockHeight: lastValidBlockHeight,
      signature: signature,
    });

    return signature;
}

describe("Close an account", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CloseAccountProgram as Program<CloseAccount>;
  const payer = provider.wallet as anchor.Wallet;

  const user = Keypair.generate(); // Generate a new user keypair

  before(async () => {
    //Transfer SOL to the user account to cover rent
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: user.publicKey,
      lamports: 2 * LAMPORTS_PER_SOL,
    });

    await createAndProcessTransaction(payer.payer, transferInstruction, [
      payer.payer,
    ]);
  });

  // Derive the PDA for the user's account.
  const [userAccount, userAccountBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("USER"), payer.publicKey.toBuffer()],
    program.programId
  );

  it("Can create an account", async () => {
    await program.methods
      .createUser("Jacob")
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    // Fetch the account data
    const userAccountData = await program.account.userState.fetch(userAccount);
    assert.equal(userAccountData.name, "Jacob");
    assert.equal(userAccountData.user.toBase58(), user.publicKey.toBase58());
    assert.notEqual(userAccountData, null);
  });

  it("Can close an Account", async () => {
    await program.methods
      .closeUser()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    // The account should no longer exist, returning null.
    try {
      const userAccountData = await program.account.userState.fetchNullable(
        userAccount
      );
      assert.equal(userAccountData, null);
    } catch (err) {
      // Won't return null and will throw an error in anchor-bankrun'
      assert.equal(err.message, `Could not find ${userAccount}`);
    }
  });
});
