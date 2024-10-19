import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";

import { HelloSolanaProgram } from "../target/types/hello_solana_program";

const IDL = require("../target/idl/hello_solana.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("hello-solana", async () => {
  // Set up Anchor with Bankrun and load the program IDL
  const context = await startAnchor(
    "",
    [{ name: "hello_solana", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);
  const program = new anchor.Program<HelloSolanaProgram>(IDL, provider);

  it("Say hello!", async () => {
    // Call the "hello" method from the IDL to run the transaction
    await program.methods.hello().accounts({}).rpc();
  });
});
