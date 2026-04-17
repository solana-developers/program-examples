import { describe, test, beforeAll } from "@jest/globals";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";
import { Program } from "@coral-xyz/anchor";

const IDL = require("../program/target/idl/transfer_sol_poseidon_program.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("Transfer SOL (Poseidon)", () => {
  let context: any;
  let provider: BankrunProvider;
  let program: Program;
  let sender: Keypair;
  let recipient: Keypair;

  beforeAll(async () => {
    sender = Keypair.generate();
    recipient = Keypair.generate();

    context = await startAnchor(
      "",
      [{ name: "transfer_sol_poseidon_program", programId: PROGRAM_ID }],
      [
        {
          address: sender.publicKey,
          info: {
            lamports: 10 * LAMPORTS_PER_SOL,
            data: Buffer.alloc(0),
            owner: SystemProgram.programId,
            executable: false,
          },
        },
      ]
    );

    provider = new BankrunProvider(context);
    program = new Program(IDL, provider);
  });

  test("Transfer SOL", async () => {
    const amount = LAMPORTS_PER_SOL;

    const senderBalanceBefore = await context.banksClient.getBalance(
      sender.publicKey
    );
    const recipientBalanceBefore = await context.banksClient.getBalance(
      recipient.publicKey
    );

    await program.methods
      .transferSol(amount)
      .accounts({
        sender: sender.publicKey,
        recipient: recipient.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc();

    const senderBalanceAfter = await context.banksClient.getBalance(
      sender.publicKey
    );
    const recipientBalanceAfter = await context.banksClient.getBalance(
      recipient.publicKey
    );

    expect(senderBalanceAfter).toBeLessThan(senderBalanceBefore);
    expect(recipientBalanceAfter).toBe(recipientBalanceBefore + amount);
  });
});
