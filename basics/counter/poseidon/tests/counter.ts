import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import type { Counter } from "../target/types/counter";

describe("counter_anchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.Counter as Program<Counter>;
  const [counter, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("")],
    program.programId
  );

  it("Initialize Counter", async () => {
    await program.methods
      .initializeCounter()
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();

    const currentCount = await program.account.counterState.fetch(counter);

    assert(
      currentCount.count.toNumber() === 0,
      "Expected initialized count to be 0"
    );
  });

  it("Increment Counter", async () => {
    await program.methods.incrementCounter().accounts({ counter }).rpc();

    const currentCount = await program.account.counterState.fetch(counter);

    assert(currentCount.count.toNumber() === 1, "Expected  count to be 1");
  });

  it("Increment Counter Again", async () => {
    await program.methods.incrementCounter().accounts({ counter }).rpc();

    const currentCount = await program.account.counterState.fetch(counter);

    assert(currentCount.count.toNumber() === 2, "Expected  count to be 2");
  });
});
