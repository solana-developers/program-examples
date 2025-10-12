import {
  PublicKey,
  Transaction,
  TransactionInstruction,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { assert } from "chai";
import { test } from "mocha";
import { start } from "solana-bankrun";
import BN from "bn.js";
describe("Program derived address tests", () => {
  const programId = new PublicKey(
    "8TpdLD58VBWsdzxRi2yRcmKJD9UcE2GuUrBwsyCwpbUN",
  );

  test("Test Cpi Transfer Works", async () => {
    const context = await start(
      [
        {
          programId,
          name: "./program/target/deploy/pinocchio_program_derived_address",
        },
      ],
      [],
    );

    const client = context.banksClient;
    const payer = context.payer;
    const blockhash = context.lastBlockhash;

    const creator = Keypair.generate();

    const [pagePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("page_visits"), creator.publicKey.toBuffer()],
      programId,
    );

    const ixs = [
      new TransactionInstruction({
        programId,
        keys: [
          { pubkey: creator.publicKey, isSigner: true, isWritable: true },
          { pubkey: pagePda, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: Buffer.from([0]),
      }),
    ];

    await getSol(creator.publicKey, payer, blockhash, client);

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(...ixs);

    tx.sign(creator);

    await client.simulateTransaction(tx, "finalized");
    const processedTxn = await client.processTransaction(tx);

    assert(processedTxn.logMessages[0].startsWith(`Program ${programId}`));
    assert(
      processedTxn.logMessages[processedTxn.logMessages.length - 1] ===
        `Program ${programId} success`,
    );

    const pageData = await client.getAccount(pagePda);

    assert(pageData?.data.length == 16, "Invalid page length"); // Check the size of the account
    assert(
      (await client.getRent()).isExempt(BigInt(pageData.lamports), BigInt(16)),
    );
  });

  test("Test Cpi Transfer Works", async () => {
    const user = Keypair.generate();
    const creator = PublicKey.unique();
    const [pagePda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("page_visits"), creator.toBuffer()],
      programId,
    );

    // The first value is the bump, the elements after that are padding, and the rest is page_count
    let pageDataBuffer = Uint8Array.from([
      bump,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
    ]);
    const context = await start(
      [
        {
          programId,
          name: "./program/target/deploy/pinocchio_program_derived_address",
        },
      ],
      [
        {
          address: pagePda,
          info: {
            executable: false,
            owner: programId,
            lamports: 1 * LAMPORTS_PER_SOL,
            data: pageDataBuffer,
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
          { pubkey: user.publicKey, isSigner: true, isWritable: true },
          { pubkey: creator, isSigner: false, isWritable: false },
          { pubkey: pagePda, isSigner: false, isWritable: true },
          {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
          },
        ],
        data: Buffer.from([1]),
      }),
    ];

    await getSol(user.publicKey, payer, blockhash, client);

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(...ixs);

    tx.sign(user);

    await client.simulateTransaction(tx, "finalized");
    const processedTxn = await client.processTransaction(tx);

    assert(processedTxn.logMessages[0].startsWith(`Program ${programId}`));
    assert(
      processedTxn.logMessages[processedTxn.logMessages.length - 1] ===
        `Program ${programId} success`,
    );

    const pageData = await client.getAccount(pagePda);
    assert(
      pageData?.data.toString() ==
        Uint8Array.from([
          bump,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          1,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
        ]).toString(),
    );
  });
});

const getSol = async (
  to: PublicKey,
  payer: Keypair,
  blockhash: string,
  client: any,
) => {
  const configs: any = {
    fromPubkey: payer.publicKey,
    lamports: 1 * LAMPORTS_PER_SOL,
    toPubkey: to,
  };

  const ix = [SystemProgram.transfer(configs)];

  const tx = new Transaction();
  tx.recentBlockhash = blockhash;
  tx.add(...ix);
  tx.sign(payer);

  await client.simulateTransaction(tx, "finalized");
  await client.processTransaction(tx);
};
