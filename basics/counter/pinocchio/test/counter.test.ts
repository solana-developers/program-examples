import {
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
} from "@solana/web3.js";
import { assert } from "chai";
import { test } from "mocha";
import { ProgramTestContext, start } from "solana-bankrun";

describe("Pinocchio-Counter", () => {
  const programId = new PublicKey(
    "8TpdLD58VBWsdzxRi2yRcmKJD9UcE2GuUrBwsyCwpbUN",
  );

  test("Test Create counter ix works", async () => {
    const context = await start(
      [{ name: "program/target/deploy/pinocchio_counter", programId }],
      [],
    );

    const client = context.banksClient;
    const payer = context.payer;
    const blockhash = context.lastBlockhash;

    const counter = PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      programId,
    )[0];

    console.log(
      "Payer::",
      payer.publicKey.toString(),
      "\nCounter",
      counter.toString(),
    );
    const ixs = [
      new TransactionInstruction({
        programId,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: counter, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: Buffer.from([0]),
      }),
    ];

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(...ixs);
    tx.sign(payer);

    await client.simulateTransaction(tx);
    await client.processTransaction(tx);
  });

  test("Test Increment counter ix works", async () => {
    const counter = PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      programId,
    )[0];

    let counterData = Buffer.alloc(
      8,
      Uint8Array.from([0, 0, 0, 0, 0, 0, 0, 0]), // The count value being 0
    );

    const context = await start(
      [{ name: "pinocchio_counter", programId }],
      [
        {
          address: counter,
          info: {
            lamports: 1000_0000,
            owner: programId,
            executable: false,
            data: counterData, // TODO
          },
        },
      ],
    );

    const client = context.banksClient;
    const payer = context.payer;
    const blockhash = context.lastBlockhash;

    const ixs = [
      new TransactionInstruction({
        programId,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: counter, isSigner: false, isWritable: true },
        ],
        data: Buffer.from([1]),
      }),
    ];

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(...ixs);
    tx.sign(payer);

    await client.simulateTransaction(tx);
    await client.processTransaction(tx);

    const counterAcc = await client.getAccount(counter);

    assert(
      counterAcc.data.toString() ==
        Uint8Array.from([1, 0, 0, 0, 0, 0, 0, 0]).toString(),
    );
  });
});
