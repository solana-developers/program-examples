import { Buffer } from "node:buffer";
import { describe, test } from "node:test";
import {
  PublicKey,
  Transaction,
  TransactionInstruction,  LAMPORTS_PER_SOL, Keypair} from "@solana/web3.js";
import * as borsh from "borsh";
import { LiteSVM } from 'litesvm';

describe("custom-instruction-data", async () => {
  console.log("custom-instruction-data");
});
