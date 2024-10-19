import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { assert } from "chai";
import { startAnchor } from "solana-bankrun";
import type { CounterProgram } from "../target/types/counter_program";

const IDL = require("../target/idl/counter_program.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("counter_program", async () => {
  // Configure the client to use the anchor-bankrun
  const context = await startAnchor(
    "",
    [{ name: "counter_program", programId: PROGRAM_ID }],
    []
  );

  const provider = new BankrunProvider(context);

  const payer = provider.wallet as anchor.Wallet;

  const program = new anchor.Program<CounterProgram>(IDL, provider);

  // Generate a new keypair for the counter account
  const counterKeypair = new Keypair();

  it("Initialize Counter", async () => {
    await program.methods
      .initializeCounter()
      .accounts({
        state: counterKeypair.publicKey,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterKeypair])
      .rpc();

    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    assert(
      currentCount.count.toNumber() === 0,
      "Expected initialized count to be 0"
    );
  });

  it("Increment Counter", async () => {
    await program.methods
      .increment()
      .accounts({ state: counterKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    assert(currentCount.count.toNumber() === 1, "Expected count to be 1");
  });

  it("Decrement Counter", async () => {
    await program.methods
      .decrement()
      .accounts({ state: counterKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    assert(
      currentCount.count.toNumber() === 0,
      "Expected count to be 0 after decrement"
    );
  });

  it("Decrement Counter Below Zero (Should Fail)", async () => {
    try {
      await program.methods
        .decrement()
        .accounts({ state: counterKeypair.publicKey })
        .rpc();

      assert.fail("Decrementing below zero should fail.");
    } catch (err) {
      assert(
        err.message.includes("Account balance is below zero"),
        "Expected failure on decrementing below zero"
      );
    }
  });

  it("Multiple Increments and Decrements", async () => {
    await program.methods
      .increment()
      .accounts({ state: counterKeypair.publicKey })
      .rpc();

    await program.methods
      .increment()
      .accounts({ state: counterKeypair.publicKey })
      .rpc();

    let currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );
    assert(
      currentCount.count.toNumber() === 2,
      "Expected count to be 2 after two increments"
    );

    await program.methods
      .decrement()
      .accounts({ state: counterKeypair.publicKey })
      .rpc();

    currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );
    assert(
      currentCount.count.toNumber() === 1,
      "Expected count to be 1 after one decrement"
    );
  });

  it("Fail to Increment with Invalid Signer", async () => {
    const invalidKeypair = Keypair.generate();

    try {
      await program.methods
        .increment()
        .accounts({ state: counterKeypair.publicKey })
        .signers([invalidKeypair]) // Invalid signer
        .rpc();

      assert.fail("Increment should have failed with an invalid signer");
    } catch (err) {
      assert(
        err.message.includes("Signature verification failed"),
        "Expected signature verification failure"
      );
    }
  });
});
