import { describe, it } from "mocha";
import {
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
} from "@solana/web3.js";
import { assert } from "chai";
import { start } from "solana-bankrun";

describe("CreateAccountWithSteel", function () {
  let context: any;
  let client: any;
  let payer: any;
  const PROGRAM_ID = PublicKey.unique();

  const NEW_ACCOUNT_SEED = Buffer.from("newaccount");

  before(async function () {
    // load program in solana-bankrun
    context = await start(
      [{ name: "new_account_program", programId: PROGRAM_ID }],
      []
    );
    client = context.banksClient;
    payer = context.payer;
  });

  it("It creates new account", async function () {
    const blockhash = context.lastBlockhash;

    const [newAccountPDA, bump] = PublicKey.findProgramAddressSync(
      [NEW_ACCOUNT_SEED],
      PROGRAM_ID
    );

    // set up your instruction first.
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: newAccountPDA, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from([0]),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    // Now process the transaction
    const transaction = await client.processTransaction(tx);

    assert(transaction.logMessages[3].startsWith(`Program log: new account is created`));
  });

});