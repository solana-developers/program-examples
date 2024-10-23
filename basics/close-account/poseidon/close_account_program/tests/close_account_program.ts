import assert from "node:assert";
import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import type { CloseAccountProgram } from "../target/types/close_account_program";

describe("Close an account", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CloseAccountProgram as Program<CloseAccountProgram>;
  const payer = provider.wallet as anchor.Wallet;

  // Derive the PDA for the user's account.
  const [userAccountAddress] = PublicKey.findProgramAddressSync(
    [Buffer.from("USER"), payer.publicKey.toBuffer()],
    program.programId
  );

  it("can create an account", async () => {
    const userId = anchor.BN(76362)

    await program.methods
      .createUser(userId)
      .accounts({
        user: payer.publicKey,
      })
      .rpc();

    // Fetch the account data
    const userAccount = await program.account.userAccount.fetch(
      userAccountAddress
    );
    assert.notEqual(userAccount, null)
  });

  it("can close an account", async () => {
    await program.methods
      .closeUser()
      .accounts({
        user: payer.publicKey
      })
      .rpc();

    // The account should no longer exist, returning null.
    const userAccount = await program.account.userAccount.fetchNullable(
      userAccountAddress
    );
    assert.equal(userAccount, null);
  });
});
