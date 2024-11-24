import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { CounterProgram } from "../target/types/counter_program";

describe("counter_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;

  // Generate a new keypair for the counter account
  const counterKeypair = new Keypair();

  it("Initializes the Counter", async () => {
    // Initialize the counter state
    await program.methods
      .initializeCounter()
      .accounts({
        state: counterKeypair.publicKey,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterKeypair])
      .rpc();

    // Fetch the current state of the counter
    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    // Assert that the counter was initialized to 0
    assert(
      currentCount.count.toNumber() === 0,
      "Expected initialized count to be 0"
    );
  });

  it("Increments the Counter", async () => {
    // Call the increment method
    await program.methods
      .increment()
      .accounts({
        state: counterKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the updated counter state
    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    // Assert that the counter was incremented to 1
    assert(currentCount.count.toNumber() === 1, "Expected count to be 1");
  });

  it("Increments the Counter Again", async () => {
    // Increment the counter again
    await program.methods
      .increment()
      .accounts({
        state: counterKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the updated counter state
    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    // Assert that the counter was incremented to 2
    assert(currentCount.count.toNumber() === 2, "Expected count to be 2");
  });

  it("Decrements the Counter", async () => {
    // Decrement the counter
    await program.methods
      .decrement()
      .accounts({
        state: counterKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the updated counter state
    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    // Assert that the counter was decremented to 1
    assert(
      currentCount.count.toNumber() === 1,
      "Expected count to be 1 after decrement"
    );
  });

  it("Handles Multiple Increments and Decrements", async () => {
    // Perform multiple increments
    for (let i = 0; i < 3; i++) {
      await program.methods
        .increment()
        .accounts({
          state: counterKeypair.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    }

    let currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );
    assert(
      currentCount.count.toNumber() === 4,
      "Expected count to be 4 after multiple increments"
    );

    // Perform multiple decrements
    for (let i = 0; i < 2; i++) {
      await program.methods
        .decrement()
        .accounts({
          state: counterKeypair.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    }

    currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );
    assert(
      currentCount.count.toNumber() === 2,
      "Expected count to be 2 after multiple decrements"
    );
  });

  it("Prevents Decrement Below Zero", async () => {
    // Decrement the counter until it reaches zero
    await program.methods
      .decrement()
      .accounts({
        state: counterKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    await program.methods
      .decrement()
      .accounts({
        state: counterKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    // Assert that counter is 0 now
    assert(
      currentCount.count.toNumber() === 0,
      "Expected count to be 0 after decrements"
    );

    // Try decrementing below zero
    try {
      await program.methods
        .decrement()
        .accounts({
          state: counterKeypair.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      assert.fail("Expected the decrement to fail when count is 0");
    } catch (error) {
      // Expected error to be thrown, as decrementing below zero isn't allowed
      assert.isTrue(
        error.toString().includes("arithmetic underflow"),
        "Expected arithmetic underflow error"
      );
    }
  });
});
