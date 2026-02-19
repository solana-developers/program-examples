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
  console.log("PDA Rent-Payer");
});
