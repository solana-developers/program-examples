import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { BN } from "bn.js";
import * as borsh from "borsh";
import { assert, expect } from "chai";
import { describe, test } from 'node:test';
import { LiteSVM } from 'litesvm';

describe("Favorites Solana Pinocchio", () => {
  console.log("Favorites Solana Pinocchio");
});
