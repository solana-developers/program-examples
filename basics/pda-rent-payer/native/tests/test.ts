import { Buffer } from "node:buffer";
import { describe, test } from "node:test";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,  LAMPORTS_PER_SOL} from "@solana/web3.js";
import * as borsh from "borsh";
import { LiteSVM } from 'litesvm';

describe("PDA Rent-Payer", async () => {
  const PROGRAM_ID = PublicKey.unique();
  const svm = new LiteSVM();
  svm.addProgramFromFile(PROGRAM_ID, 'tests/fixtures/pda_rent_payer_program.so');
  
  const payer = Keypair.generate();
  svm.airdrop(payer.publicKey, BigInt(10 * LAMPORTS_PER_SOL));

  class Assignable {
    constructor(properties) {
      for (const [key, value] of Object.entries(properties)) {
        this[key] = value;
      }
    }
  }

  const MyInstruction = {
    InitRentVault: 0,
    CreateNewAccount: 1,
  } as const;

  class InitRentVault extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(InitRentVaultSchema, this));
    }
  }
  const InitRentVaultSchema = new Map([
    [
      InitRentVault,
      {
        kind: "struct",
        fields: [
          ["instruction", "u8"],
          ["fund_lamports", "u64"],
        ],
      },
    ],
  ]);

  class CreateNewAccount extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(CreateNewAccountSchema, this));
    }
  }
  const CreateNewAccountSchema = new Map([
    [
      CreateNewAccount,
      {
        kind: "struct",
        fields: [["instruction", "u8"]],
      },
    ],
  ]);

  function deriveRentVaultPda() {
    const pda = PublicKey.findProgramAddressSync(
      [Buffer.from("rent_vault")],
      PROGRAM_ID,
    );
    console.log(`PDA: ${pda[0].toBase58()}`);
    return pda;
  }

  test("Initialize the Rent Vault", async () => {
    const blockhash = svm.latestBlockhash();
    const [rentVaultPda, _] = deriveRentVaultPda();
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: rentVaultPda, isSigner: false, isWritable: true },
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: new InitRentVault({
        instruction: MyInstruction.InitRentVault,
        fund_lamports: 1000000000,
      }).toBuffer(),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer);

    svm.sendTransaction(tx);
  });

  test("Create a new account using the Rent Vault", async () => {
    const blockhash = svm.latestBlockhash();
    const newAccount = Keypair.generate();
    const [rentVaultPda, _] = deriveRentVaultPda();
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: newAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: rentVaultPda, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: new CreateNewAccount({
        instruction: MyInstruction.CreateNewAccount,
      }).toBuffer(),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, newAccount); // Add instruction and Sign the transaction

    // Now we process the transaction
    svm.sendTransaction(tx);
  });
});
