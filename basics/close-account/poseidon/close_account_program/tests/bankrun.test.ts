import assert from "node:assert";
import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";
import type { CloseAccountProgram } from "../target/types/close_account_program";

const IDL = require("../target/idl/close_account_program.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("Close an account", async () => {
  // Configure the client to use the local cluster.
  const context = await startAnchor(
    "",
    [{ name: "close_account", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;
  const program = new anchor.Program<CloseAccountProgram>(IDL, provider);
  // Derive the PDA for the user's account.
  const [userAccountAddress] = PublicKey.findProgramAddressSync(
    [Buffer.from("USER"), payer.publicKey.toBuffer()],
    program.programId
  );

  it("Can create an account", async () => {
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
     assert.notEqual(userAccount, null);
  });

  it("Can close an Account", async () => {
    await program.methods
      .closeUser()
      .accounts({
        user: payer.publicKey,
      })
      .rpc();

    // The account should no longer exist, returning null.
    try {
      const userAccount = await program.account.userAccount.fetchNullable(
        userAccountAddress
      );
      assert.equal(userAccount, null);
    } catch (err) {
      // Won't return null and will throw an error in anchor-bankrun'
      assert.equal(err.message, `Could not find ${userAccountAddress}`);
    }
  });
});
