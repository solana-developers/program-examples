import { Buffer } from "node:buffer";
import {
  Connection,
  Keypair,
  SystemProgram,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";

describe("CPI Example", () => {
  console.log("CPI Example");
});
