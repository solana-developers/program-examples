import { Buffer } from "node:buffer";
import { describe, test } from "node:test";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { start } from "solana-bankrun";

describe("PDA Rent-Payer", async () => {
  console.log("PDA Rent-Payer");
});
