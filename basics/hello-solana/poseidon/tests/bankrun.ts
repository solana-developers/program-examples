import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";
import { HelloSolanaProgram } from "../target/types/hello_solana_program";

const IDL = require("../target/idl/hello_solana_program.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("hello-solana", async () => {
  // Configure the Anchor provider & load the program IDL for anchor-bankrun
  // The IDL gives you a typescript module
  const context = await startAnchor(
    "",
    [{ name: "hello_solana_program", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);

  const program = new anchor.Program<HelloSolanaProgram>(IDL, provider);

  it("Say hello!", async () => {
    await program.methods.helloSolana().accounts({}).rpc();
  });
});
