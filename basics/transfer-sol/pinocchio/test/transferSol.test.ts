import {
  PublicKey,
  Transaction,
  TransactionInstruction,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import pkg from "@solana/web3.js";
import { assert } from "chai";
import { test } from "mocha";
import { ProgramTestContext, start } from "solana-bankrun";
import BN from "bn.js";
describe("Transfer Sol Tests", () => {
  const programId = new PublicKey(
    "8TpdLD58VBWsdzxRi2yRcmKJD9UcE2GuUrBwsyCwpbUN",
  );

  test("Test Cpi Transfer Works", async () => {
    const context = await start(
      [{ programId, name: "./program/target/deploy/pinocchio_transfer_sol" }],
      [],
    );

    const client = context.banksClient;
    const payer = context.payer;
    const blockhash = context.lastBlockhash;

    const param = { amount: LAMPORTS_PER_SOL };

    const ix_data_buffer = Buffer.concat([
      Uint8Array.from([0]),
      Uint8Array.from(new BN(param.amount).toArray("le", 8)),
    ]);
    const sender = Keypair.generate();
    const receiver = PublicKey.unique();

    const initialCredit: any = {
      fromPubkey: payer.publicKey,
      lamports: 1.5 * LAMPORTS_PER_SOL,
      toPubkey: sender.publicKey,
    };

    const ix1 = [SystemProgram.transfer(initialCredit)];

    const tx1 = new Transaction();
    tx1.recentBlockhash = blockhash;
    tx1.add(...ix1);

    tx1.sign(payer);

    await client.simulateTransaction(tx1);
    await client.processTransaction(tx1);
    const ix2 = [
      new TransactionInstruction({
        programId,
        keys: [
          { pubkey: sender.publicKey, isSigner: true, isWritable: true },
          { pubkey: receiver, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: ix_data_buffer,
      }),
    ];

    const tx2 = new Transaction();
    tx2.recentBlockhash = blockhash;
    tx2.add(...ix2);
    tx2.feePayer = payer.publicKey;
    tx2.sign(sender, payer);

    const senderBalanceBefore = await client.getBalance(
      sender.publicKey,
      "finalized",
    );
    await client.simulateTransaction(tx2, "finalized");
    const processedTxn = await client.processTransaction(tx2);

    assert(
      processedTxn.logMessages[0].startsWith(`Program ${programId.toString()}`),
    );

    assert(
      processedTxn.logMessages[processedTxn.logMessages.length - 1] ==
        `Program ${programId.toString()} success`,
    );

    const senderBalanceAfter = await client.getBalance(
      sender.publicKey,
      "finalized",
    );
    const receiverBalanceAfter = await client.getBalance(receiver, "finalized");

    assert(receiverBalanceAfter.toString() == param.amount.toString());

    assert(
      new BN(senderBalanceBefore)
        .sub(new BN(senderBalanceAfter))
        .eq(new BN(param.amount)),
    );
  });
});
