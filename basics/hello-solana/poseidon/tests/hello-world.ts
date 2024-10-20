import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorldProgram } from "../target/types/hello_world_program";
import {  startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { PublicKey } from "@solana/web3.js";

const IDL = require("../target/idl/hello_world_program.json");

const PROGRAM_ID = new PublicKey(IDL.address);

describe("hello-world", async() => {
  // Configure the client to use the local cluster.
  const context = await startAnchor(
    "",
    [{ name: "counter_program", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);
  const program = anchor.workspace.HelloWorldProgram as Program<HelloWorldProgram>;
  

  it("init", async () => {
    const txid = await program.methods
      .initialize()
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Initialize tx:", txid);
  });
  
});

