import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import type { TokenSwap } from "../target/types/token_swap";
import {
  type TestValues,
  createValues,
  expectRevert,
  mintingTokens,
} from "./utils";
import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";

const IDL = require("../target/idl/token_swap.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("Create pool", async () => {
 const context = await startAnchor(
   "",
   [{ name: "token_swap", programId: PROGRAM_ID }],
   []
 );

 const provider = new BankrunProvider(context);

const connection = provider.connection;

 const payer = provider.wallet as anchor.Wallet;

 const program = new anchor.Program<TokenSwap>(IDL, provider);

  let values: TestValues;

  beforeEach(async () => {
    values = createValues();
    const id = new anchor.BN(values.id)
    const fee = values.fee
    await program.methods
      .createAmm(id, fee)
      .accounts({
        // admin: values.admin.publicKey 
      })
      .rpc();

    await mintingTokens({
      connection,
      creator: values.admin,
      mintAKeypair: values.mintAKeypair,
      mintBKeypair: values.mintBKeypair,
    });
  });

  it("Creation", async () => {
    const id = new anchor.BN(values.id);
    await program.methods
      .createPool(id)
      .accounts({
        // amm: values.ammKey,
        // pool: values.poolKey,
        // poolAuthority: values.poolAuthority,
        // mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        // poolAccountA: values.poolAccountA,
        // poolAccountB: values.poolAccountB,
      })
      .rpc({ skipPreflight: true });
  });

  it("Invalid mints", async () => {
    values = createValues({
      mintBKeypair: values.mintAKeypair,
      poolKey: PublicKey.findProgramAddressSync(
        [
          Buffer.alloc(values.id),
          values.mintAKeypair.publicKey.toBuffer(),
          values.mintBKeypair.publicKey.toBuffer(),
        ],
        program.programId
      )[0],
      poolAuthority: PublicKey.findProgramAddressSync(
        [
          Buffer.alloc(values.id),
          values.mintAKeypair.publicKey.toBuffer(),
          values.mintBKeypair.publicKey.toBuffer(),
          Buffer.from("authority"),
        ],
        program.programId
      )[0],
    });
 const id = new anchor.BN(values.id);
    await expectRevert(
      program.methods
        .createPool(id)
        .accounts({
          // amm: values.ammKey,
          // pool: values.poolKey,
          // poolAuthority: values.poolAuthority,
          // mintLiquidity: values.mintLiquidity,
          mintA: values.mintAKeypair.publicKey,
          mintB: values.mintBKeypair.publicKey,
          // poolAccountA: values.poolAccountA,
          // poolAccountB: values.poolAccountB,
        })
        .rpc()
    );
  });
});
