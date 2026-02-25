import { readFileSync } from "node:fs";
import { describe, test } from "node:test";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { LiteSVM } from "litesvm";
import {
  createCloseUserInstruction,
  createCreateUserInstruction,
} from "../ts/index.ts";

describe("Close Account!", () => {
  // Load the program keypair
  const programKeypairPath = new URL(
    "./fixtures/close_account_native_program-keypair.json",
    // @ts-ignore
    import.meta.url,
  ).pathname;
  const programKeypairData = JSON.parse(
    readFileSync(programKeypairPath, "utf-8"),
  );
  const programKeypair = Keypair.fromSecretKey(
    new Uint8Array(programKeypairData),
  );
  const PROGRAM_ID = programKeypair.publicKey;

  const litesvm = new LiteSVM();
  const payer = Keypair.generate();

  // Load the program
  const programPath = new URL(
    "./fixtures/close_account_native_program.so",
    // @ts-ignore
    import.meta.url,
  ).pathname;
  litesvm.addProgramFromFile(PROGRAM_ID, programPath);

  // Fund the payer account
  litesvm.airdrop(payer.publicKey, BigInt(100000000000));

  const testAccountPublicKey = PublicKey.findProgramAddressSync(
    [Buffer.from("USER"), payer.publicKey.toBuffer()],
    PROGRAM_ID,
  )[0];

  test("Create the account", () => {
    const ix = createCreateUserInstruction(
      testAccountPublicKey,
      payer.publicKey,
      PROGRAM_ID,
      "Jacob",
    );

    const tx = new Transaction().add(ix);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(payer);

    litesvm.sendTransaction(tx);
  });

  test("Close the account", () => {
    const ix = createCloseUserInstruction(
      testAccountPublicKey,
      payer.publicKey,
      PROGRAM_ID,
    );

    const tx = new Transaction().add(ix);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(payer);

    litesvm.sendTransaction(tx);
  });
});
