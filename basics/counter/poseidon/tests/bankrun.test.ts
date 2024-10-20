import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { assert } from "chai";
import { startAnchor } from "solana-bankrun";
import type { CounterProgramPoseidon } from "../target/types/counter_program_poseidon";

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

  const program = new anchor.Program<CounterProgramPoseidon>(IDL, provider);

  // Generate a new keypair for the counter account
  const counterKeypair = new Keypair();

  it("Initialize Counter", async () => {
    await program.methods
      .initializeCounter()
      .accounts({
        counter: counterKeypair.publicKey,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
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
      .incrementCounter()
      .accounts({ counter: counterKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    assert(currentCount.count.toNumber() === 1, "Expected count to be 1");
  });

  it("Decrement Counter", async () => {
    await program.methods
      .decrementCounter()
      .accounts({ counter: counterKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.counterState.fetch(
      counterKeypair.publicKey
    );

    assert(
      currentCount.count.toNumber() === 0,
      "Expected count to be 0 after decrement"
    );
  });


});