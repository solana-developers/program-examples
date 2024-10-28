import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterProgram } from "../target/types/counter_program";
import { assert } from "chai";

describe("Counter program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;
  const counterState = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("count")],
    program.programId
  )[0];

  it("Create and initialize counter state", async () => {
    const txid = await program.methods
      .initializeCounter()
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("initialize tx:", txid);

    const counterStateAccount = await program.account.counterState.fetch(counterState);
    assert.ok(counterStateAccount.count.eq(new anchor.BN(0)));
  });

  it("Increment count", async () => {
    const txid = await program.methods.increment().accounts({}).rpc();

    console.log("increment tx:", txid);

    const counterStateAccount = await program.account.counterState.fetch(counterState);
    assert.ok(counterStateAccount.count.eq(new anchor.BN(1)));
  });

  it("Decrement count", async () => {
    const txid = await program.methods.decrement().accounts({}).rpc();

    console.log("decrement tx:", txid);

    const counterStateAccount = await program.account.counterState.fetch(counterState);
    assert.ok(counterStateAccount.count.eq(new anchor.BN(0)));
  });
});