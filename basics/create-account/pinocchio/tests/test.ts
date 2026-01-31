import { describe, test } from "node:test";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { LiteSVM } from 'litesvm';

describe("Create a system account", async () => {
  const PROGRAM_ID = PublicKey.unique();
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/create_account_pinocchio_program.so');
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  test("Create the account via a cross program invocation", async () => {
    const newKeypair = Keypair.generate();
    const blockhash = svm.latestBlockhash();

    const ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: newKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.alloc(0),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, newKeypair);

    svm.sendTransaction(tx);
  });

  test("Create the account via direct call to system program", async () => {
    const newKeypair = Keypair.generate();
    const blockhash = svm.latestBlockhash();

    const ix = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: newKeypair.publicKey,
      lamports: LAMPORTS_PER_SOL,
      space: 0,
      programId: SystemProgram.programId,
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, newKeypair);

    svm.sendTransaction(tx);
    console.log(
      `Account with public key ${newKeypair.publicKey} successfully created`,
    );
  });
});
