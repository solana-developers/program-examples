import {
  Blockhash,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { BN } from "bn.js";
import * as borsh from "borsh";
import { assert, expect } from "chai";
import { describe, test } from "mocha";
import { BanksClient, ProgramTestContext, start } from "solana-bankrun";

describe("Favorites Solana Pinocchio", () => {
  console.log("Favorites Solana Pinocchio");
});
