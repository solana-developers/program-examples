import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  Keypair
} from '@solana/web3.js'
import { assert } from "chai";
import { CounterAnchor } from "../target/types/counter_anchor";

describe("counter_anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CounterAnchor as Program<CounterAnchor>;

  it("Test increment", async () => {
    const counterKeypair = Keypair.generate();
    const counter = counterKeypair.publicKey;

    // Initialize counter
    await program.methods
      .initializeCounter()
      .accounts({ counter, payer: program.provider.publicKey })
      .signers([counterKeypair])
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    let currentCount = (await program.account.counter.fetch(counter, "confirmed")).count.toNumber();
    assert(currentCount === 0, "Expected initialized count to be 0");

    // Increment counter
    await program.methods
      .increment()
      .accounts({ counter })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    currentCount = (await program.account.counter.fetch(counter, "confirmed")).count.toNumber();
    assert(currentCount === 1, "Expected count to be 1");

    // Increment counter
    await program.methods
      .increment()
      .accounts({ counter })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    currentCount = (await program.account.counter.fetch(counter, "confirmed")).count.toNumber();
    assert(currentCount === 2, "Expected count to be 2");
  });
});
