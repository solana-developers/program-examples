import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { CloseAccountProgram } from "../target/types/close_account_program";
import {
  BlockheightBasedTransactionConfirmationStrategy,
  PublicKey,
} from "@solana/web3.js";
import assert from "assert";

describe("close-an-account", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .CloseAccountProgram as Program<CloseAccountProgram>;
  const connection = program.provider.connection;
  const payer = Keypair.generate();

  async function airdrop(receiver: PublicKey, amount: number) {
    const sig = await program.provider.connection.requestAirdrop(
      receiver,
      amount
    );
    const blockStats = await program.provider.connection.getLatestBlockhash();
    const strategy: BlockheightBasedTransactionConfirmationStrategy = {
      signature: sig,
      blockhash: blockStats.blockhash,
      lastValidBlockHeight: blockStats.lastValidBlockHeight,
    };
    await program.provider.connection.confirmTransaction(strategy, "confirmed");
  }

  function getUserAccount(user: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("USER"), user.toBuffer()],
      program.programId
    );
  }

  it("Airdrop", async () => {
    const balanceBefore = await connection.getBalance(payer.publicKey);
    await airdrop(payer.publicKey, LAMPORTS_PER_SOL);
    const balanceAfter = await connection.getBalance(payer.publicKey);
    assert.equal(balanceAfter, balanceBefore + LAMPORTS_PER_SOL);
  });

  it("Create Account", async () => {
    const [userAccountAddress] = getUserAccount(payer.publicKey);
    const userAccountBefore = await program.account.user.fetchNullable(
      userAccountAddress,
      "confirmed"
    );
    assert.equal(userAccountBefore, null);

    await program.methods
      .createUser({
        name: "John Doe",
      })
      .accounts({
        payer: payer.publicKey,
        userAccount: userAccountAddress,
      })
      .signers([payer])
      .rpc({ commitment: "confirmed", skipPreflight: true });

    const userAccountAfter = await program.account.user.fetchNullable(
      userAccountAddress,
      "confirmed"
    );
    assert.notEqual(userAccountAfter, null);
    assert.equal(userAccountAfter.name, "John Doe");
    assert.equal(userAccountAfter.user.toBase58(), payer.publicKey.toBase58());
  });

  it("Close Account", async () => {
    const [userAccountAddress] = getUserAccount(payer.publicKey);
    const userAccountBefore = await program.account.user.fetchNullable(
      userAccountAddress,
      "confirmed"
    );
    assert.notEqual(userAccountBefore, null);

    await program.methods
      .closeUser()
      .accounts({
        user: payer.publicKey,
        userAccount: userAccountAddress,
      })
      .signers([payer])
      .rpc({ commitment: "confirmed" });

    const userAccountAfter = await program.account.user.fetchNullable(
      userAccountAddress,
      "processed"
    );
    assert.equal(userAccountAfter, null);
  });
});
