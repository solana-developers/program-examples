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

  const [counterState, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("count")],
    program.programId
  );

  it("Initialize Counter", async () => {
    await program.methods
      .initializeCounter()
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();

    const currentCount = await program.account.counter.fetch(counterState);

    assert(
      currentCount.count.toNumber() === 0,
      "Expected initialized count to be 0"
    );
  });

  it("Increment Counter", async () => {
    await program.methods.increment().accounts({}).rpc();

    const currentCount = await program.account.counter.fetch(counterState);

    assert(currentCount.count.toNumber() === 1, "Expected  count to be 1");
  });

  it("Increment Counter Again", async () => {
    await program.methods.increment().accounts({ counter: counterState }).rpc();

    const currentCount = await program.account.counter.fetch(counterState);

    assert(currentCount.count.toNumber() === 2, "Expected  count to be 2");
  });
  it("Decrement counter", async () => {
    await program.methods.decrement().accounts({}).rpc();

    const currentCount = await program.account.counter.fetch(counterState);
    assert(currentCount.count.toNumber() === 1, "Expected  count to be 1");
  });
  it("Increment and decrement multiple times", async () => {
    // Increment the counter 5 times
    for (let i = 0; i < 5; i++) {
      await program.methods.increment().accounts({}).rpc();
    }

    let currentCount = await program.account.counter.fetch(counterState);
    assert.strictEqual(
      currentCount.count.toNumber(),
      6,
      "Expected count to be 6 after 5 increments"
    );

    // Decrement the counter 4 times
    for (let i = 0; i < 4; i++) {
      await program.methods.decrement().accounts({}).rpc();
    }

    currentCount = await program.account.counter.fetch(counterState);
    assert.strictEqual(
      currentCount.count.toNumber(),
      2,
      "Expected count to be 2 after 4 decrements"
    );
  });

  it("Cannot decrement below 0", async () => {
    // Decrement the counter to 0
    await program.methods.decrement().accounts({}).rpc();
    await program.methods.decrement().accounts({}).rpc();
    const currentCount = await program.account.counter.fetch(counterState);
    assert.strictEqual(
      currentCount.count.toNumber(),
      0,
      "Expected count to be 0 after multiple decrements"
    );
  });
});
