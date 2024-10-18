import { describe, it } from "mocha";
import {
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
} from "@solana/web3.js";
import { assert } from "chai";
import { start } from "solana-bankrun";

describe("hello-solana", function () {
  let context: any;
  let client: any;
  let payer: any;
  const PROGRAM_ID = PublicKey.unique();

  const COUNTER_SEED = Buffer.from("counter");

  before(async function () {
    // load program in solana-bankrun
    context = await start(
      [{ name: "steel_program", programId: PROGRAM_ID }],
      []
    );
    client = context.banksClient;
    payer = context.payer;
  });

  it("Say hello!", async function () {
    const blockhash = context.lastBlockhash;

    const [counterPDA, bump] = PublicKey.findProgramAddressSync(
      [COUNTER_SEED],
      PROGRAM_ID
    );

    // We set up our instruction first.
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: counterPDA, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from([0]),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    // Now we process the transaction
    const transaction = await client.processTransaction(tx);

    assert(transaction.logMessages[3].startsWith(`Program log: Initialized`));
  });

  it("Increment", async function () {
    const blockhash = context.lastBlockhash;

    const [counterPDA, bump] = PublicKey.findProgramAddressSync(
      [COUNTER_SEED],
      PROGRAM_ID
    );

    // We set up our instruction first.
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: counterPDA, isSigner: false, isWritable: true },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from([1]),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    // Now we process the transaction
    const transaction = await client.processTransaction(tx);

    assert(transaction.logMessages[1] === `Program log: Increment`);
    assert(transaction.logMessages[2] === `Program log: Counter Value 1`);
    assert(transaction.logMessages.length === 5);
  });
});
